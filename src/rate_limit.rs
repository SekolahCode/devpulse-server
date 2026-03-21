use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

/// Simple sliding-window rate limiter keyed by a string (e.g. API key).
/// Uses only std — no extra crate required.
pub struct RateLimiter {
    store:          Mutex<HashMap<String, Vec<Instant>>>,
    max_per_window: usize,
    window:         Duration,
}

impl RateLimiter {
    pub fn new(max_per_window: usize, window_secs: u64) -> Self {
        Self {
            store:          Mutex::new(HashMap::new()),
            max_per_window,
            window:         Duration::from_secs(window_secs),
        }
    }

    /// Returns `true` if the request is allowed, `false` if rate-limited.
    pub fn check(&self, key: &str) -> bool {
        let mut store = self.store.lock().unwrap_or_else(|e| e.into_inner());
        let now       = Instant::now();
        let window    = self.window;
        let entry     = store.entry(key.to_string()).or_default();

        // Evict timestamps outside the current window
        entry.retain(|&t| now.duration_since(t) < window);

        if entry.len() >= self.max_per_window {
            false
        } else {
            entry.push(now);
            true
        }
    }
}
