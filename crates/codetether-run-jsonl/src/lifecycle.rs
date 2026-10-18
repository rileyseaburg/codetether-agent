//! Run lifecycle output.
use super::event::RunEvent;
use anyhow::Result;
use codetether_provider_core::Usage;

/// Write a started record to standard output.
/// # Arguments
/// * `session_id` - Session identity when available.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_started("session-1")?;
/// # Ok(()) }
/// ```
pub fn write_started(session_id: &str) -> Result<()> {
    super::writer::write_stdout(&RunEvent::Started { session_id })
}

/// Write a completed record to standard output.
/// # Arguments
/// * `session_id` - Session identity when available.
/// * `response` - Final or partial response text.
/// * `usage` - Optional provider token counts.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_completed(Some("session-1"), "done", None)?;
/// # Ok(()) }
/// ```
pub fn write_completed(
    session_id: Option<&str>,
    response: &str,
    usage: Option<&Usage>,
) -> Result<()> {
    super::writer::write_stdout(&RunEvent::Completed {
        session_id,
        response,
        usage,
    })
}

/// Write a failed record to standard output.
/// # Arguments
/// * `error` - Failure description.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_failed("request failed")?;
/// # Ok(()) }
/// ```
pub fn write_failed(error: impl AsRef<str>) -> Result<()> {
    write_failed_response(error.as_ref(), None)
}

/// Write a failed response record to standard output.
/// # Arguments
/// * `response` - Final or partial response text.
/// * `error` - Failure description.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_failed_response("request failed", Some("partial"))?;
/// # Ok(()) }
/// ```
pub fn write_failed_response(error: &str, response: Option<&str>) -> Result<()> {
    super::writer::write_stdout(&RunEvent::Failed { error, response })
}
