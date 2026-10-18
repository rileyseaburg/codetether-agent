//! Compact bus payloads before retaining them in TUI state.

use crate::bounded;

/// Retention budget for field max bytes.
pub const FIELD_MAX_BYTES: usize = 512;
/// Retention budget for kind max bytes.
pub const KIND_MAX_BYTES: usize = 64;
/// Retention budget for summary max bytes.
pub const SUMMARY_MAX_BYTES: usize = 512;
/// Retention budget for detail max bytes.
pub const DETAIL_MAX_BYTES: usize = 8 * 1024;

/// Bound a bus-event field.
/// # Arguments
/// Field text.
/// # Returns
/// Bounded field text.
/// # Examples
/// ```
/// assert_eq!(codetether_view_retention::bus::field("ok"), "ok");
/// ```
pub fn field(input: &str) -> String {
    bounded(input, FIELD_MAX_BYTES, "bus field")
}

/// Bound a bus-event kind label.
/// # Arguments
/// Kind text.
/// # Returns
/// Bounded kind text.
/// # Examples
/// ```
/// assert_eq!(codetether_view_retention::bus::kind("event"), "event");
/// ```
pub fn kind(input: &str) -> String {
    bounded(input, KIND_MAX_BYTES, "bus kind")
}

/// Bound a bus-event summary.
/// # Arguments
/// Summary text.
/// # Returns
/// Bounded summary text.
/// # Examples
/// ```
/// assert_eq!(codetether_view_retention::bus::summary("ok"), "ok");
/// ```
pub fn summary(input: &str) -> String {
    bounded(input, SUMMARY_MAX_BYTES, "bus summary")
}

/// Bound a bus-event detail payload.
/// # Arguments
/// Detail text and diagnostic label.
/// # Returns
/// Bounded detail text.
/// # Examples
/// ```
/// assert_eq!(codetether_view_retention::bus::detail("ok", "result"), "ok");
/// ```
pub fn detail(input: &str, label: &str) -> String {
    bounded(input, DETAIL_MAX_BYTES, label)
}

/// Combine tool header and output under the bus detail budget.
/// # Arguments
/// Header, output and diagnostic label.
/// # Returns
/// Combined bounded detail.
/// # Examples
/// ```
/// assert_eq!(codetether_view_retention::bus::tool_detail("tool", "ok", "result"), "tool\nok");
/// ```
pub fn tool_detail(header: &str, output: &str, label: &str) -> String {
    detail(&format!("{header}\n{}", detail(output, label)), label)
}
