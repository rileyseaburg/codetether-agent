//! Shared stream idle deadline used by drains and acceptance errors.
use std::time::Duration;

/// Maximum gap between consecutive stream chunks before the stream is
/// considered stalled. GLM-5.2 can think 60-90 s before the first token,
/// Codex uses a five-minute per-activity timeout for Responses streams.
pub const IDLE_TIMEOUT: Duration = Duration::from_secs(300);
