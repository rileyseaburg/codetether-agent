//! Persistent telemetry for A2A message handling.

use crate::telemetry::record_persistent;
use std::time::Duration;

#[allow(clippy::too_many_arguments)]
pub(super) fn record_a2a_message_telemetry(
    tool_name: &str,
    task_id: &str,
    blocking: bool,
    prompt: &str,
    duration: Duration,
    success: bool,
    output: Option<String>,
    error: Option<String>,
) {
    let record = crate::telemetry::A2AMessageRecord {
        tool_name: tool_name.to_string(),
        task_id: task_id.to_string(),
        blocking,
        prompt: prompt.to_string(),
        duration_ms: duration.as_millis() as u64,
        success,
        output,
        error,
        timestamp: chrono::Utc::now(),
    };
    let _ = record_persistent(
        "a2a_message",
        &serde_json::to_value(&record).unwrap_or_default(),
    );
}
