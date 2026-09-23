use kite_agent_toolkit::{classify_http, Outcome};

fn main() {
    let status = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(429);
    match classify_http(status, 0) {
        Outcome::Completed(_) => println!("completed"),
        Outcome::Retry { after, attempt } => println!("retry attempt={} after_ms={}", attempt, after.as_millis()),
        Outcome::Rejected { code } => println!("rejected code={}", code),
    }
}
