/// Redis-backed sliding-window rate limiter.
/// Uses an atomic Lua script (INCR + EXPIRE in one round-trip) to avoid the
/// race condition where concurrent requests both see count == 1 and both set
/// their own TTL, leaving the key without an expiry.
/// Fails open (allows request) on Redis errors so an outage never blocks ingest.
use deadpool_redis::Pool as RedisPool;

pub struct RateLimiter {
    pool:           RedisPool,
    max_per_window: usize,
    window_secs:    u64,
}

/// Lua script: atomically increment and set expiry only on the first request.
/// Returns the current count after incrementing.
const RATE_LIMIT_SCRIPT: &str = r#"
local count = redis.call('INCR', KEYS[1])
if count == 1 then
    redis.call('EXPIRE', KEYS[1], ARGV[1])
end
return count
"#;

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

        let count: i64 = match deadpool_redis::redis::cmd("EVAL")
            .arg(RATE_LIMIT_SCRIPT)
            .arg(1)                   // number of keys
            .arg(&redis_key)
            .arg(self.window_secs)
            .query_async(&mut *conn)
            .await
        {
            Ok(c)  => c,
            Err(_) => return true,
        };

        count <= self.max_per_window as i64
    }
}
