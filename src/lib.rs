//! Dependency-free reliability primitives for payment-capable agents.
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome<T> { Completed(T), Retry { after: Duration, attempt: u32 }, Rejected { code: &'static str } }

#[derive(Debug, Default)]
pub struct IdempotencyCache { entries: HashMap<String, String> }

impl IdempotencyCache {
    pub fn new() -> Self { Self::default() }
    pub fn record(&mut self, key: &str, response: &str) -> Result<(), &'static str> {
        if self.entries.contains_key(key) { return Err("idempotency key already used"); }
        self.entries.insert(key.to_owned(), response.to_owned()); Ok(())
    }
    pub fn replay(&self, key: &str) -> Option<&str> { self.entries.get(key).map(String::as_str) }
}

pub fn backoff(attempt: u32, base_ms: u64, max_ms: u64) -> Duration {
    let shift = attempt.min(20);
    Duration::from_millis(base_ms.saturating_mul(1u64 << shift).min(max_ms))
}

pub fn classify_http(status: u16, attempt: u32) -> Outcome<()> {
    match status {
        200..=299 => Outcome::Completed(()),
        408 | 425 | 429 | 500..=599 => Outcome::Retry { after: backoff(attempt, 250, 30_000), attempt: attempt + 1 },
        402 => Outcome::Rejected { code: "payment_required" },
        _ => Outcome::Rejected { code: "upstream_rejected" },
    }
}

pub fn is_retryable(status: u16) -> bool { matches!(status, 408 | 425 | 429 | 500..=599) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn retries_transient_responses() { assert!(matches!(classify_http(429, 2), Outcome::Retry { .. })); }
    #[test] fn caps_backoff() { assert_eq!(backoff(20, 250, 30_000), Duration::from_millis(30_000)); }
    #[test] fn identifies_transient_statuses() { assert!(is_retryable(503)); assert!(!is_retryable(402)); }
    #[test] fn prevents_duplicate_submission() { let mut c = IdempotencyCache::new(); assert!(c.record("k", "ok").is_ok()); assert!(c.record("k", "again").is_err()); assert_eq!(c.replay("k"), Some("ok")); }
}
