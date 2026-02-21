use sqlx::PgPool;
use std::time::Duration;

/// Background task that runs every 24 hours and deletes events older than
/// `retention_days` days.  The app binary self-configures this via the
/// `EVENT_RETENTION_DAYS` environment variable (default: 90).
pub async fn run(pool: PgPool, retention_days: i64) {
    tracing::info!("🗑️  Retention worker started (keep last {} days)", retention_days);

    loop {
        tokio::time::sleep(Duration::from_secs(24 * 3600)).await;
        prune(&pool, retention_days).await;
    }
}

/// Run once immediately (called at startup so the first prune doesn't wait 24 h).
pub async fn prune(pool: &PgPool, retention_days: i64) {
    match sqlx::query!(
        "DELETE FROM events WHERE created_at < NOW() - make_interval(days => $1)",
        retention_days as i32
    )
    .execute(pool)
    .await
    {
        Ok(r)  => tracing::info!("Retention: pruned {} old event(s)", r.rows_affected()),
        Err(e) => tracing::error!("Retention prune failed: {}", e),
    }
}
