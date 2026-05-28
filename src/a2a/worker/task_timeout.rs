//! Task timeout parsing helpers.

pub(super) fn task_timeout_secs(
    task: &serde_json::Value,
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> u64 {
    task_u64(task, "task_timeout_seconds")
        .or_else(|| task_u64(task, "timeout_secs"))
        .or_else(|| {
            super::metadata_u64(
                metadata,
                &["task_timeout_seconds", "timeout_secs", "timeout"],
            )
        })
        .unwrap_or(1200)
        .clamp(60, 604800)
}

fn task_u64(task: &serde_json::Value, key: &str) -> Option<u64> {
    let value = super::task_value(task, key)?;
    if let Some(value) = value.as_u64() {
        return Some(value);
    }
    if let Some(value) = value.as_i64()
        && value >= 0
    {
        return Some(value as u64);
    }
    value
        .as_str()
        .and_then(|value| value.trim().parse::<u64>().ok())
}
