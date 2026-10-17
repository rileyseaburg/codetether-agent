//! Agent-authored context-reset markers with shared audit recording.
//!
//! [ContextResetTool] validates a summary, records its acceptance and returns
//! the stable [RESET_MARKER_PREFIX] marker. [format_reset_marker] also serves
//! callers that already have a summary. This crate does not alter transcripts.
mod audit;
mod execute;
mod marker;
mod schema;
mod tool;
pub use marker::{RESET_MARKER_PREFIX, format_reset_marker};
/// Return an audited reset marker from an agent-authored summary.
///
/// # Examples
/// ```rust
/// let _tool=codetether_context_reset::ContextResetTool;
/// ```
pub struct ContextResetTool;
#[cfg(test)]
mod tests;
