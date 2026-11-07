//! Synthetic catalog reproducing descriptions with literal asterisks.
use codetether_provider_core::ToolDefinition;
use codetether_tool_catalog::list_tools_bootstrap_output;
use serde_json::{Value, json};

pub(super) fn catalog(query: Value) -> Value {
    let tools = [
        ("glob", "Find files matching *.rs"),
        ("rustyroad", "Call rustyroad_* tools"),
        ("websearch", "Search the web"),
        ("webfetch", "Fetch a web page"),
    ]
    .into_iter()
    .map(|(name, description)| ToolDefinition {
        name: name.into(),
        description: description.into(),
        parameters: json!({}),
    })
    .collect::<Vec<_>>();
    serde_json::from_str(&list_tools_bootstrap_output(&tools, &query)).unwrap()
}
