//! Inspect only added patch lines for stub markers.
use serde_json::Value;
pub(super) fn detect(tool_input: &Value) -> Option<String> {
    let patch = tool_input.get("patch").and_then(Value::as_str)?;
    let added_lines = patch
        .lines()
        .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
        .map(|line| line.trim_start_matches('+'))
        .collect::<Vec<_>>()
        .join("\n");
    if added_lines.is_empty() {
        None
    } else {
        super::marker::stub_marker_in_text(&added_lines)
            .map(|marker| format!("patch additions contains stub marker \"{marker}\""))
    }
}
