//! Frontend limits for bounded tool argument and output previews.
//!
//! These defaults keep large JSON and file payloads out of repeated full-frame
//! formatting. Frontends can import the same limits for compact preview views.
/// Largest argument payload eligible for pretty JSON formatting.
pub const TOOL_ARGS_PRETTY_JSON_MAX_BYTES: usize = 16_000;
/// Maximum lines in a compact argument preview.
pub const TOOL_ARGS_PREVIEW_MAX_LINES: usize = 10;
/// Maximum bytes in a compact argument preview.
pub const TOOL_ARGS_PREVIEW_MAX_BYTES: usize = 6_000;
/// Maximum lines in a compact tool-output preview.
pub const TOOL_OUTPUT_PREVIEW_MAX_LINES: usize = 5;
/// Maximum bytes in a compact tool-output preview.
pub const TOOL_OUTPUT_PREVIEW_MAX_BYTES: usize = 4_000;
