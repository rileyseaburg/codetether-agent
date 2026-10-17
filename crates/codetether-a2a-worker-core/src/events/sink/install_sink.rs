//! Worker event sink install sink.
use super::*;

/// Installs the sink used for the currently executing task.
///
/// # Examples
///
/// # Arguments
/// * `sink` - Replacement callback, or `None` to clear it.
/// # Returns
/// Returns unit after attempting to replace the sink.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::events::sink::install_sink;
/// install_sink(None);
/// ```
pub fn install_sink(sink: Option<EventSink>) {
    if let Ok(mut slot) = ACTIVE_SINK.write() {
        *slot = sink;
    }
}
