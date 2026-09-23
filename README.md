# Kite Agent Toolkit

This crate collects the small reliability decisions that tend to be duplicated in payment-facing agents. It classifies upstream responses, calculates capped exponential backoff, and keeps a process-local idempotency cache so a retry cannot submit the same logical operation twice.

```bash
cargo test
cargo run -- 429
```

The crate deliberately has no network client and no signing key. It sits below an agent's transport adapter, making retry and replay behaviour testable without contacting Kite or an upstream service.
