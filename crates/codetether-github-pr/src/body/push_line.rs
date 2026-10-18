//! PR body push line.
pub(super) fn push_line(lines: &mut Vec<String>, label: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        lines.push(format!("- {label}: `{value}`"));
    }
}
