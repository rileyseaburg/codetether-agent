//! Filter and alphabetize the compact name-and-description catalog.
use codetether_provider_core::ToolDefinition;
use serde_json::{Value, json};
pub(super) fn list(tools: &[ToolDefinition], tool_input: &Value) -> String {
    let query = tool_input
        .get("query")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase());

    let mut listed: Vec<Value> = tools
        .iter()
        .filter(|t| {
            if let Some(q) = &query {
                t.name.to_ascii_lowercase().contains(q)
                    || t.description.to_ascii_lowercase().contains(q)
            } else {
                true
            }
        })
        .map(|t| {
            json!({
                "name": t.name,
                "description": t.description,
            })
        })
        .collect();

    listed.sort_by(|a, b| {
        let an = a["name"].as_str().unwrap_or_default();
        let bn = b["name"].as_str().unwrap_or_default();
        an.cmp(bn)
    });

    serde_json::to_string_pretty(&json!({
        "mode": "list",
        "count": listed.len(),
        "tools": listed,
        "hint": "Call list_tools with {\"tool\":\"<name>\"} to fetch full parameter schema for one tool."
    }))
    .unwrap_or_else(|_| "{\"mode\":\"list\",\"error\":\"serialization_failed\"}".to_string())
}
