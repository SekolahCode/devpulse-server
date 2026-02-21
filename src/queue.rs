use deadpool_redis::{redis::AsyncCommands, Pool as RedisPool};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::IngestPayload;

pub const QUEUE_KEY: &str = "devpulse:events";

// The job structure pushed into Redis
#[derive(Debug, Serialize, Deserialize)]
pub struct EventJob {
    pub project_id: Uuid,
    pub payload:    IngestPayload,
}

// Push a job onto the queue
pub async fn push_job(
    redis_pool: &RedisPool,
    job: EventJob,
) -> Result<(), String> {
    let mut conn = redis_pool.get().await
        .map_err(|e| e.to_string())?;

    let job_json = serde_json::to_string(&job)
        .map_err(|e| e.to_string())?;

    conn.lpush::<_, _, ()>(QUEUE_KEY, job_json).await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Pop a job from the queue (blocks up to 5 seconds)
pub async fn pop_job(
    redis_pool: &RedisPool,
) -> Option<EventJob> {
    let mut conn = redis_pool.get().await.ok()?;

    let result: Option<(String, String)> = deadpool_redis::redis::cmd("BRPOP")
        .arg(QUEUE_KEY)
        .arg(5)
        .query_async(&mut *conn)
        .await
        .unwrap_or(None);

    let (_, job_json) = result?;
    serde_json::from_str(&job_json).ok()
}
