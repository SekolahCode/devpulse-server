use deadpool_redis::{redis::AsyncCommands, Pool as RedisPool};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::IngestPayload;

pub const QUEUE_KEY: &str = "devpulse:events";
pub const DEAD_LETTER_KEY: &str = "devpulse:events:dead";

// The job structure pushed into Redis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventJob {
    pub project_id: Uuid,
    pub payload:    IngestPayload,
}

// A dead-lettered job: the original job plus the error reason and a timestamp.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeadLetterEntry {
    pub job:        EventJob,
    pub error:      String,
    pub failed_at:  String, // ISO-8601
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

/// Push a failed job to the dead-letter queue with the error reason.
/// Dead-lettered jobs are kept for manual inspection or replay.
/// The list is capped at 1 000 entries to prevent unbounded growth.
pub async fn dead_letter(
    redis_pool: &RedisPool,
    job: EventJob,
    error: String,
) {
    let entry = DeadLetterEntry {
        job,
        error,
        failed_at: chrono::Utc::now().to_rfc3339(),
    };

    let json = match serde_json::to_string(&entry) {
        Ok(j)  => j,
        Err(e) => {
            tracing::error!("Failed to serialise dead-letter entry: {}", e);
            return;
        }
    };

    match redis_pool.get().await {
        Ok(mut conn) => {
            // LPUSH then LTRIM keeps the list bounded (newest entries at the front).
            let result: Result<(), _> = deadpool_redis::redis::pipe()
                .cmd("LPUSH").arg(DEAD_LETTER_KEY).arg(&json).ignore()
                .cmd("LTRIM").arg(DEAD_LETTER_KEY).arg(0).arg(999).ignore()
                .query_async(&mut *conn)
                .await;

            if let Err(e) = result {
                tracing::error!("Failed to write to dead-letter queue: {}", e);
            } else {
                tracing::warn!("Job moved to dead-letter queue ({})", DEAD_LETTER_KEY);
            }
        }
        Err(e) => {
            tracing::error!("Could not get Redis connection for dead-letter queue: {}", e);
        }
    }
}
