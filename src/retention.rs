use sqlx::PgPool;
use std::time::Duration;

pub async fn run(pool: PgPool, retention_days: i64) {
    tracing::info!("🗑️  Retention worker started (keep last {} days)", retention_days);
    loop {
        tokio::time::sleep(Duration::from_secs(24 * 3600)).await;
        prune(&pool, retention_days).await;
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
