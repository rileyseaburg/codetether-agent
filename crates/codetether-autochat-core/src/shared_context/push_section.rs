//! Relay context push section.
use super::*;

pub(super) fn push_section(lines: &mut Vec<String>, title: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }
    lines.push(format!("{title}:"));
    lines.extend(values.iter().map(|v| format!("- {v}")));
}
