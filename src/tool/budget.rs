//! Per-tool-output byte budget resolution.
//!
//! Keeps a single tool result well under typical provider context windows.

/// Default per-tool-output byte budget. Tunable at runtime via the
/// `CODETETHER_TOOL_OUTPUT_MAX_BYTES` environment variable. Chosen to keep a
/// single tool result well under typical provider context windows even after
/// JSON re-encoding overhead.
pub const DEFAULT_TOOL_OUTPUT_MAX_BYTES: usize = 64 * 1024;

/// Resolve the current tool-output byte budget from env, falling back to
/// [`DEFAULT_TOOL_OUTPUT_MAX_BYTES`]. Invalid values fall back to the default.
pub fn tool_output_budget() -> usize {
    std::env::var("CODETETHER_TOOL_OUTPUT_MAX_BYTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_TOOL_OUTPUT_MAX_BYTES)
}
