//! Render one VS Code language model tool summary line.

use super::summary::LmToolSummary;

pub(super) fn render_tool(tool: &LmToolSummary) -> String {
    let reference = tool
        .reference
        .as_ref()
        .map(|name| format!(" / `{name}`"))
        .unwrap_or_default();
    format!(
        "- `{}`{}: {}{}\n",
        tool.name,
        reference,
        tool.description,
        required_label(&tool.required)
    )
}

fn required_label(required: &[String]) -> String {
    if required.is_empty() {
        String::new()
    } else {
        format!(" Required: {}.", required.join(", "))
    }
}
