use crate::alerts::{fire_alerts, AlertContext};
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

use crate::queue::{pop_job, EventJob};
use tokio::sync::broadcast;

pub async fn run(redis_pool: RedisPool, pg_pool: PgPool, event_tx: broadcast::Sender<String>) {
    tracing::info!("⚙️  Worker started — listening on queue");

    loop {
        match pop_job(&redis_pool).await {
            Some(job) => {
                let pool = pg_pool.clone();
                let tx   = event_tx.clone();
                tokio::spawn(async move {
                    if let Err(e) = process(job, &pool, &tx).await {
                        tracing::error!("Worker failed to process job: {}", e);
                    }
                });
            }
            None => continue, // BRPOP timeout — loop
        }
    }
}

async fn process(job: EventJob, pool: &PgPool, event_tx: &broadcast::Sender<String>) -> Result<(), sqlx::Error> {
    let payload = &job.payload;

    // 1. Build title
    let title = match &payload.exception {
        Some(e) => format!(
            "{}: {}",
            e.r#type.as_deref().unwrap_or("Error"),
            e.message
        ),
        None => payload.message
            .clone()
            .unwrap_or_else(|| "Unknown error".to_string()),
    };

    // 2. Fingerprint (MD5 of type:message)
    let fingerprint_raw = match &payload.exception {
        Some(e) => format!("{}:{}", e.r#type.as_deref().unwrap_or(""), e.message),
        None    => title.clone(),
    };
    let fingerprint = format!("{:x}", md5::compute(&fingerprint_raw));
    let level       = payload.level.as_deref().unwrap_or("error").to_string();
    let environment = payload.environment.clone().unwrap_or_else(|| "production".into());

    // 3. Upsert issue (xmax trick detects new row vs update)
    let issue = sqlx::query!(
        r#"
        INSERT INTO issues (project_id, fingerprint, title, level)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (project_id, fingerprint)
        DO UPDATE SET
            event_count = issues.event_count + 1,
            last_seen   = NOW(),
            status      = CASE
                WHEN issues.status = 'resolved' THEN 'unresolved'
                ELSE issues.status
            END
        RETURNING id, event_count,
            (xmax = 0)                              AS is_new_row,
            (xmax != 0 AND status = 'resolved')     AS is_regression
        "#,
        job.project_id,
        fingerprint,
        title,
        level
    )
    .fetch_one(pool)
    .await?;

    // 4. Store raw event (including environment)
    let payload_json = serde_json::to_value(&job.payload)
        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
    sqlx::query!(
        "INSERT INTO events (issue_id, project_id, payload, context, environment)
         VALUES ($1, $2, $3, $4, $5)",
        issue.id,
        job.project_id,
        payload_json,
        job.payload.context,
        environment
    )
    .execute(pool)
    .await?;

    let is_new        = issue.is_new_row.unwrap_or(false);
    let is_regression = issue.is_regression.unwrap_or(false);

    // 5. Fire alerts only on new issue or regression (prevents alert spam)
    if is_new || is_regression {
        let project = sqlx::query!(
            "SELECT name FROM projects WHERE id = $1",
            job.project_id
        )
        .fetch_one(pool)
        .await?;

        fire_alerts(pool, AlertContext {
            issue_id:      issue.id,
            project_id:    job.project_id,
            project_name:  project.name,
            title:         title.clone(),
            level:         level.clone(),
            is_regression,
        })
        .await;
    }

    // 6. Broadcast to WebSocket subscribers
    let broadcast_msg = serde_json::json!({
        "type":          "new_event",
        "issue_id":      issue.id,
        "project_id":    job.project_id,
        "title":         title,
        "level":         level,
        "is_new":        is_new,
        "is_regression": is_regression,
    });
    let _ = event_tx.send(broadcast_msg.to_string());

    tracing::info!("✅ Event processed → issue: {}", issue.id);
    Ok(())
}
