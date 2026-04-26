//! Windows apps listing for computer use.

pub fn handle_list_apps() -> anyhow::Result<crate::tool::ToolResult> {
    let script = r#"
$p=Get-Process|Where-Object {$_.MainWindowHandle -ne 0 -and $_.MainWindowTitle}
$p|Select-Object @{n='app';e={$_.ProcessName}},@{n='pid';e={$_.Id}},@{n='window_title';e={$_.MainWindowTitle}},@{n='hwnd';e={$_.MainWindowHandle.ToInt64()}}|ConvertTo-Json -Depth 3
"#;
    let value = super::ps::run(script)?;
    let apps = match value {
        serde_json::Value::Array(items) => items,
        serde_json::Value::Null => Vec::new(),
        other => vec![other],
    };
    Ok(super::response::success_result(serde_json::json!({
        "platform": "Windows",
        "count": apps.len(),
        "apps": apps
    })))
}
