//! Bounded display excerpts of formatted tool arguments.
use crate::format_tool_call_arguments;

/// Build a size-limited preview for tool arguments.
#[allow(dead_code)]
///
/// # Arguments
/// * `tool_name` - Tool name used for argument formatting.
/// * `arguments` - JSON or literal arguments.
/// * `max_lines` - Maximum preview lines.
/// * `max_bytes` - Maximum preview bytes.
///
/// # Returns
/// The formatted excerpt and its truncation flag.
///
/// # Examples
/// ```
/// assert_eq!(codetether_tool_preview::build_tool_arguments_preview("tool", "hello", 1, 3), ("hel".into(), true));
/// ```
pub fn build_tool_arguments_preview(
    tool_name: &str,
    arguments: &str,
    max_lines: usize,
    max_bytes: usize,
) -> (String, bool) {
    let formatted = format_tool_call_arguments(tool_name, arguments);
    codetether_util::text_preview::build_text_preview(&formatted, max_lines, max_bytes)
}
