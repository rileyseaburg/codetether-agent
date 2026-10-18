//! Human-readable tool arguments and bounded text previews.
//!
//! [format_tool_call_arguments] expands small JSON objects for display.
//! [build_tool_arguments_preview] then applies explicit line and byte limits.
mod format;
mod preview;
pub use format::format_tool_call_arguments;
pub use preview::build_tool_arguments_preview;
