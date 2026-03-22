/// Redis-backed sliding-window rate limiter.
/// Uses a fixed-window INCR approach: first INCR sets TTL, subsequent INCRs
/// count within that window. Fails open (allows request) on Redis errors so
/// an outage never blocks ingest.
use deadpool_redis::Pool as RedisPool;

pub struct RateLimiter {
    pool:           RedisPool,
    max_per_window: usize,
    window_secs:    u64,
}

impl RateLimiter {
    pub fn new(pool: RedisPool, max_per_window: usize, window_secs: u64) -> Self {
        Self { pool, max_per_window, window_secs }
    }

    /// Returns `true` if the request is allowed, `false` if rate-limited.
    pub async fn check(&self, key: &str) -> bool {
        let mut conn = match self.pool.get().await {
            Ok(c)  => c,
            Err(_) => return true, // fail open — don't block ingest on Redis error
        };

        let redis_key = format!("devpulse:rl:{}", key);

        let count: i64 = match deadpool_redis::redis::cmd("INCR")
            .arg(&redis_key)
            .query_async(&mut *conn)
            .await
        {
            Ok(c)  => c,
            Err(_) => return true,
        };

        // Only set expiry on the first request in the window
        if count == 1 {
            let _: Result<(), _> = deadpool_redis::redis::cmd("EXPIRE")
                .arg(&redis_key)
                .arg(self.window_secs)
                .query_async(&mut *conn)
                .await;
        }

        count <= self.max_per_window as i64
    }
}
