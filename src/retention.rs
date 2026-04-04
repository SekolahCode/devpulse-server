use sqlx::PgPool;
use std::time::Duration;

/// Numeric key for the PostgreSQL advisory lock that guards the retention job.
/// Must be unique across all advisory locks in the app.
const RETENTION_LOCK_KEY: i64 = 0x6465_7670_756c_7365; // "devpulse" as i64

pub async fn run(pool: PgPool, retention_days: i64) {
    tracing::info!("Retention worker started (keep last {} days)", retention_days);
    loop {
        tokio::time::sleep(Duration::from_secs(24 * 3600)).await;

        // Try to acquire a PostgreSQL session-level advisory lock (non-blocking).
        // Only one instance across the cluster will succeed; others skip this cycle.
        let locked: bool = match sqlx::query_scalar!(
            "SELECT pg_try_advisory_lock($1)",
            RETENTION_LOCK_KEY
        )
        .fetch_one(&pool)
        .await
        {
            Ok(Some(v)) => v,
            Ok(None)    => false,
            Err(e) => {
                tracing::error!("Retention: advisory lock query failed: {}", e);
                continue;
            }
        };

        if !locked {
            tracing::debug!("Retention: lock held by another instance, skipping");
            continue;
        }

        prune(&pool, retention_days).await;

        // Release the lock so the next instance can acquire it on the next cycle.
        if let Err(e) = sqlx::query_scalar!(
            "SELECT pg_advisory_unlock($1)",
            RETENTION_LOCK_KEY
        )
        .fetch_one(&pool)
        .await
        {
            tracing::warn!("Retention: failed to release advisory lock: {}", e);
        }
    }
}

pub async fn prune(pool: &PgPool, retention_days: i64) {
    // Events older than retention window
    match sqlx::query!(
        "DELETE FROM events WHERE created_at < NOW() - make_interval(days => $1)",
        retention_days as i32
    )
    .execute(pool)
    .await
    {
        Ok(r)  => tracing::info!("Retention: pruned {} old event(s)", r.rows_affected()),
        Err(e) => tracing::error!("Retention: event prune failed: {}", e),
    }

    // Resolved/ignored issues with no events in the last retention window
    // (the events were already pruned, so the issue is stale)
    match sqlx::query!(
        r#"
        DELETE FROM issues
        WHERE status IN ('resolved', 'ignored')
          AND last_seen < NOW() - make_interval(days => $1)
        "#,
        retention_days as i32
    )
    .execute(pool)
    .await
    {
        Ok(r)  => tracing::info!("Retention: pruned {} stale issue(s)", r.rows_affected()),
        Err(e) => tracing::error!("Retention: issue prune failed: {}", e),
    }

    // Orphaned issue_users rows (cascade should handle this, but be explicit)
    if let Err(e) = sqlx::query!(
        "DELETE FROM issue_users WHERE issue_id NOT IN (SELECT id FROM issues)"
    )
    .execute(pool)
    .await
    {
        tracing::error!("Retention: issue_users cleanup failed: {}", e);
    }
}
