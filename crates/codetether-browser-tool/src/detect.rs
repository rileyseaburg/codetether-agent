//! Browser runtime availability detection response.
use super::ToolResult;

pub(super) fn detect_result() -> ToolResult {
    match codetether_browser::detect_browser() {
        Some(path) => {
            let output = serde_json::json!({
                "found": true,
                "executable_path": path.display().to_string(),
                "platform": std::env::consts::OS,
            });
            ToolResult {
                output: serde_json::to_string_pretty(&output).unwrap_or_default(),
                success: true,
                metadata: Default::default(),
            }
        }
        None => {
            let output = serde_json::json!({
                "found": false,
                "platform": std::env::consts::OS,
                "hint": "No external browser executable is required; browserctl uses the native TetherScript backend.",
            });
            ToolResult {
                output: serde_json::to_string_pretty(&output).unwrap_or_default(),
                success: true,
                metadata: Default::default(),
            }
        }
    }
}
