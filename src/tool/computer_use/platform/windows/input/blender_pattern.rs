//! Blender object-name pattern construction.

pub fn select_pattern(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.starts_with('*') && trimmed.ends_with('*') {
        trimmed.to_string()
    } else {
        format!("*{trimmed}*")
    }
}

pub fn matched(name: &str, selected: &[String]) -> bool {
    selected.iter().any(|item| item.contains(name.trim()))
}
