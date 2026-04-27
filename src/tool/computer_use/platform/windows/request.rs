//! Windows app request handling for computer use.

pub fn handle_request_app(
    input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let app = input.app.as_deref().unwrap_or("unspecified");
    Ok(super::response::success_result(serde_json::json!({
        "approved": false,
        "app": app,
        "scope": "none",
        "reason": "request_app_not_enforced"
    })))
}
