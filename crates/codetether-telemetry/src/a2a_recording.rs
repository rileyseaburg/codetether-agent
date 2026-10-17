//! Persistent telemetry for A2A message handling.

use crate::record_persistent;
use std::time::Duration;

/// Persist one completed A2A message observation.
///
/// # Arguments
///
/// Supply the operation name, task id, blocking mode, prompt, elapsed time,
/// success flag and optional output/error text.
///
/// # Returns
///
/// Returns after attempting persistence; serialization and storage failures
/// retain the existing best-effort behavior.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_telemetry::a2a_recording::record_a2a_message_telemetry;
/// record_a2a_message_telemetry(
///     "a2a_message_send", "task-1", true, "hello",
///     std::time::Duration::from_millis(20), true, Some("hi".into()), None,
/// );
/// ```
#[allow(clippy::too_many_arguments)]
pub fn record_a2a_message_telemetry(
    tool_name: &str,
    task_id: &str,
    blocking: bool,
    prompt: &str,
    duration: Duration,
    success: bool,
    output: Option<String>,
    error: Option<String>,
) {
    let record = crate::A2AMessageRecord {
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
