use crate::alerts::{fire_alerts, AlertContext};
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::Arc;

use crate::queue::{pop_job, EventJob};
use tokio::sync::Semaphore;

const MAX_CONCURRENT_JOBS: usize = 64;

pub async fn run(redis_pool: RedisPool, pg_pool: PgPool) {
    tracing::info!("⚙️  Worker started — listening on queue");

    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_JOBS));

    loop {
        match pop_job(&redis_pool).await {
            Some(job) => {
                let pool      = pg_pool.clone();
                let redis     = redis_pool.clone();
                let permit    = semaphore.clone().acquire_owned().await
                    .expect("semaphore closed");
                tokio::spawn(async move {
                    let _permit = permit;
                    if let Err(e) = process(job, &pool, &redis).await {
                        tracing::error!("Worker failed to process job: {}", e);
                    }
                });
            }
            None => continue,
        }
    }
}

async fn process(job: EventJob, pool: &PgPool, redis_pool: &RedisPool) -> Result<(), sqlx::Error> {
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

    // 2. Improved fingerprint: MD5 of "type:message:top_frame"
    //    Including the top stack frame prevents unrelated errors with the same
    //    message (e.g. "null pointer") from collapsing into one issue.
    let top_frame = payload.exception.as_ref()
        .and_then(|e| e.stacktrace.as_ref())
        .and_then(|st| st.first())
        .map(|f| format!(
            "{}:{}",
            f.file.as_deref().unwrap_or(""),
            f.line.unwrap_or(0)
        ))
        .unwrap_or_default();

    let fingerprint_raw = match &payload.exception {
        Some(e) => format!(
            "{}:{}:{}",
            e.r#type.as_deref().unwrap_or(""),
            e.message,
            top_frame
        ),
        None => title.clone(),
    };
    let fingerprint = format!("{:x}", md5::compute(&fingerprint_raw));
    let level       = payload.level.as_deref().unwrap_or("error").to_string();
    let environment = payload.environment.clone().unwrap_or_else(|| "production".into());
    let release     = payload.release.clone();

    // 3. Extract a normalised user key for deduplication
    let user_key: Option<String> = payload.user.as_ref().and_then(|u| {
        u.get("id").and_then(|v| v.as_str()).map(|s| s.to_string())
            .or_else(|| u.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()))
            .or_else(|| u.get("ip").and_then(|v| v.as_str()).map(|s| s.to_string()))
    });

    // 4. Serialize breadcrumbs
    let breadcrumbs_json = payload.breadcrumbs.as_ref()
        .and_then(|b| serde_json::to_value(b).ok());

    // 5. Atomic transaction: upsert issue + insert event
    let mut tx = pool.begin().await?;

    let issue = sqlx::query!(
        r#"
        INSERT INTO issues (project_id, fingerprint, title, level, environment, last_release)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (project_id, fingerprint)
        DO UPDATE SET
            event_count  = issues.event_count + 1,
            last_seen    = NOW(),
            environment  = EXCLUDED.environment,
            last_release = COALESCE(EXCLUDED.last_release, issues.last_release),
            status       = CASE
                WHEN issues.status = 'resolved' THEN 'unresolved'
                ELSE issues.status
            END
        RETURNING id, event_count,
            (xmax = 0)                          AS is_new_row,
            (xmax != 0 AND status = 'resolved') AS is_regression,
            first_release
        "#,
        job.project_id,
        fingerprint,
        title,
        level,
        environment,
        release.clone() as Option<String>,
    )
    .fetch_one(&mut *tx)
    .await?;

    // Set first_release if this is the first time we see this issue
    if issue.is_new_row.unwrap_or(false) && release.is_some() {
        sqlx::query!(
            "UPDATE issues SET first_release = $1 WHERE id = $2 AND first_release IS NULL",
            release.clone() as Option<String>,
            issue.id
        )
        .execute(&mut *tx)
        .await?;
    }

    // 6. Insert event
    let payload_json = serde_json::to_value(&job.payload)
        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

    sqlx::query!(
        "INSERT INTO events (issue_id, project_id, payload, context, environment, release, breadcrumbs)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        issue.id,
        job.project_id,
        payload_json,
        job.payload.context,
        environment,
        release as Option<String>,
        breadcrumbs_json
    )
    .execute(&mut *tx)
    .await?;

    // 7. Deduplicate affected users (INSERT IGNORE on issue_users)
    if let Some(ref key) = user_key {
        let inserted = sqlx::query_scalar!(
            r#"
            INSERT INTO issue_users (issue_id, user_key)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            RETURNING 1 AS "inserted!: i32"
            "#,
            issue.id,
            key
        )
        .fetch_optional(&mut *tx)
        .await?;

        if inserted.is_some() {
            // New unique user for this issue — increment the counter
            sqlx::query!(
                "UPDATE issues SET affected_users = affected_users + 1 WHERE id = $1",
                issue.id
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    let is_new        = issue.is_new_row.unwrap_or(false);
    let is_regression = issue.is_regression.unwrap_or(false);

    // 8. Fire alerts on new issue or regression
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

    // 9. Publish to Redis so all WebSocket subscribers (including other instances) receive it
    let broadcast_msg = serde_json::json!({
        "type":          "new_event",
        "issue_id":      issue.id,
        "project_id":    job.project_id,
        "title":         title,
        "level":         level,
        "is_new":        is_new,
        "is_regression": is_regression,
    })
    .to_string();

    if let Ok(mut conn) = redis_pool.get().await {
        let _: Result<(), _> = deadpool_redis::redis::cmd("PUBLISH")
            .arg("devpulse:events")
            .arg(&broadcast_msg)
            .query_async(&mut *conn)
            .await;
    }

    tracing::info!("✅ Event processed → issue: {}", issue.id);
    Ok(())
}
