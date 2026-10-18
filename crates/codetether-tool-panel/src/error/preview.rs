use super::parse_structured_tool_error;

/// Format a structured error as a multiline tool preview.
///
/// # Arguments
/// * `text` - Tool output containing a JSON error envelope.
/// # Returns
/// Tool/code header, message, missing fields and example, or `None` for an unrecognized envelope.
/// # Examples
/// ```
/// assert!(codetether_tool_panel::error::format_structured_tool_error_preview("{}").is_none());
/// ```
pub fn format_structured_tool_error_preview(text: &str) -> Option<String> {
    let parsed = parse_structured_tool_error(text)?;
    let mut lines = Vec::new();

    let mut header_bits = Vec::new();
    if let Some(tool) = parsed.tool.as_deref() {
        header_bits.push(tool.to_string());
    }
    if let Some(code) = parsed.code.as_deref() {
        header_bits.push(code.to_string());
    }
    if !header_bits.is_empty() {
        lines.push(header_bits.join(" • "));
    }

    if let Some(message) = parsed.message.as_deref() {
        lines.push(message.to_string());
    }

    if !parsed.missing_fields.is_empty() {
        lines.push(format!("Missing: {}", parsed.missing_fields.join(", ")));
    }

    if let Some(example) = parsed.example.as_deref() {
        lines.push("Example:".to_string());
        lines.extend(example.lines().map(ToOwned::to_owned));
    }

    Some(lines.join("\n"))
}
