use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pool(database_url: &str) -> PgPool {
    // 30 connections comfortably covers 64 concurrent workers plus API traffic.
    let max = std::env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30u32);

    PgPoolOptions::new()
        .max_connections(max)
        .connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}
