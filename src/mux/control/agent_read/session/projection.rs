//! Semantic mux-session response projection.

use serde_json::{Value, json};

use crate::mux::MuxRuntimeStatus;

pub(super) fn render(runtime: &MuxRuntimeStatus, indexed: &Value) -> String {
    json!({
        "transport": "mux_session",
        "session_title": runtime.session_title,
        "status": if runtime.processing { "working" } else { "idle" },
        "current_tool": runtime.current_tool,
        "principal": &runtime.principal,
        "output": excerpts(indexed)
    })
    .to_string()
}

pub(super) fn excerpts(indexed: &Value) -> String {
    let Some(documents) = indexed["documents"].as_array() else {
        return String::new();
    };
    documents
        .iter()
        .rev()
        .take(2)
        .rev()
        .filter_map(|document| document["excerpt"].as_str())
        .collect::<Vec<_>>()
        .join("\n")
}
