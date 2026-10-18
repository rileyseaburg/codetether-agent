//! Execution of the ralph_status MCP tool.
use super::*;
pub(super) fn call(args: Value) -> Result<CallToolResult> {
    let prd_path = args
        .get("prd_path")
        .and_then(|v| v.as_str())
        .unwrap_or("prd.json");

    let mut result = json!({});

    // Read PRD
    if let Ok(content) = std::fs::read_to_string(prd_path) {
        if let Ok(prd) = serde_json::from_str::<serde_json::Value>(&content) {
            let stories = prd.get("user_stories").and_then(|s| s.as_array());
            let passed = stories
                .map(|arr| {
                    arr.iter()
                        .filter(|s| s.get("passes").and_then(|v| v.as_bool()).unwrap_or(false))
                        .count()
                })
                .unwrap_or(0);
            let total = stories.map(|arr| arr.len()).unwrap_or(0);

            result["prd"] = prd;
            result["summary"] = json!({
                "passed": passed,
                "total": total,
                "progress_pct": if total > 0 { (passed * 100) / total } else { 0 },
            });
        }
    } else {
        result["prd_error"] = json!("prd.json not found");
    }

    // Read progress.txt
    let progress_path = std::path::Path::new(prd_path)
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .join("progress.txt");
    if let Ok(progress) = std::fs::read_to_string(&progress_path) {
        result["progress"] = json!(progress);
    }

    let text = serde_json::to_string_pretty(&result).unwrap_or_default();

    Ok(CallToolResult {
        content: vec![ToolContent::Text { text }],
        is_error: false,
    })
}
