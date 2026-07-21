//! Durable semantic transcript projection for a registered mux TUI.

use anyhow::Result;
use serde_json::{Value, json};

use crate::mux::registry::MuxRecord;

pub(super) async fn read(record: &MuxRecord) -> Result<Option<String>> {
    let Some(runtime) = record.state.runtime.as_ref() else {
        return Ok(None);
    };
    let Some(window) = record
        .state
        .windows
        .iter()
        .find(|window| window.id == record.state.active_window)
    else {
        return Ok(None);
    };
    let path = window
        .workspace
        .join(".codetether-agent/sessions/recall/sessions")
        .join(format!("{}.json", runtime.session_id));
    let Ok(data) = tokio::fs::read(path).await else {
        return Ok(None);
    };
    let indexed: Value = serde_json::from_slice(&data)?;
    Ok(Some(
        json!({
            "transport": "mux_session",
            "session_title": runtime.session_title,
            "status": if runtime.processing { "working" } else { "idle" },
            "current_tool": runtime.current_tool,
            "output": excerpts(&indexed)
        })
        .to_string(),
    ))
}

fn excerpts(indexed: &Value) -> String {
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

#[cfg(test)]
#[path = "session/tests.rs"]
mod tests;
