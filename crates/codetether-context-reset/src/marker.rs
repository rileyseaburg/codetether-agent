//! Stable marker format consumed by subsequent context derivation.

/// Marker prefix that identifies a `context_reset` tool output in the
/// transcript. Phase B derivations search for this literal when
/// deciding whether a user-visible summary has already landed.
pub const RESET_MARKER_PREFIX: &str = "[CONTEXT RESET]";

/// Shape the tool-result payload carrying the agent-authored summary.
///
/// # Arguments
/// * `summary` - Caller-authored summary text.
/// * `bytes` - Summary byte count shown in the marker.
/// # Returns
/// Stable marker text followed by the supplied summary.
/// # Examples
/// ```rust
/// let marker=codetether_context_reset::format_reset_marker("hello",5);
/// assert!(marker.starts_with(codetether_context_reset::RESET_MARKER_PREFIX));
/// assert!(marker.contains("bytes=5"));
/// ```
///
/// Kept as a free function so the exact wire format can be unit-tested
/// in isolation from the [`codetether_tool_core::Tool`] trait impl.
pub fn format_reset_marker(summary: &str, bytes: usize) -> String {
    format!(
        "{RESET_MARKER_PREFIX}\n\
         The agent requested a Lu et al. (arXiv:2510.06727) context \
         reset. Everything older than this point should be treated as \
         summarised by the text below. The canonical transcript is \
         preserved on disk; use `session_recall` for dropped details \
         only when the user allows history access.\n\
         \n\
         bytes={bytes}\n\
         \n\
         {summary}"
    )
}
