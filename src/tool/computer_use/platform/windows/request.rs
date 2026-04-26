//! Windows app request handling for computer use.

pub fn handle_request_app(
    input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let app = input.app.as_deref().unwrap_or("unspecified");
    let allowed = !matches!(
        app.to_ascii_lowercase().as_str(),
        "cmd" | "powershell" | "pwsh"
    );
    Ok(super::response::success_result(serde_json::json!({
        "approved": allowed,
        "app": app,
        "scope": "current_action",
        "reason": if allowed { "allowed_by_default_policy" } else { "terminal_control_blocked" }
    })))
}
