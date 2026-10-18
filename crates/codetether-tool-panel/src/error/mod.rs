//! Structured tool-error parsing and preview formatting.
//!
//! [parse_structured_tool_error] extracts a [StructuredToolError] from a JSON
//! envelope. [format_structured_tool_error_preview] produces display text.
mod model;
mod parse;
mod preview;
pub use model::StructuredToolError;
pub use parse::parse_structured_tool_error;
pub use preview::format_structured_tool_error_preview;
#[cfg(test)]
mod tests;
