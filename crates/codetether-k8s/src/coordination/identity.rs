//! Cross-process identity attached to each Kubernetes mutation claim.

use uuid::Uuid;

/// Build a unique, attributable holder identity for one mutation operation.
pub fn new_holder(operation: &str) -> String {
    let agent = env("CODETETHER_AGENT_NAME").unwrap_or_else(|| "codetether".into());
    let session = env("CODETETHER_SESSION_ID").unwrap_or_else(|| process_scope());
    let task = env("CODETETHER_TASK_ID").unwrap_or_else(|| "no-task".into());
    let nonce = Uuid::new_v4().simple().to_string();
    bounded(format!("{agent}|{session}|{task}|{operation}|{nonce}"), 240)
}

/// Stable field-manager identity for Kubernetes managed-fields attribution.
pub fn field_manager() -> String {
    let agent = env("CODETETHER_AGENT_NAME").unwrap_or_else(|| "agent".into());
    format!("codetether-{}", sanitize(&agent))
}

fn env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn process_scope() -> String {
    format!(
        "{}-{}",
        gethostname::gethostname().to_string_lossy(),
        std::process::id()
    )
}

fn sanitize(value: &str) -> String {
    bounded(
        value
            .to_ascii_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect(),
        100,
    )
}

fn bounded(mut value: String, max: usize) -> String {
    let mut end = max.min(value.len());
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    value.truncate(end);
    value
}
