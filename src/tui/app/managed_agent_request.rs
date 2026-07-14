//! Managed-agent tool request construction and dispatch.

use std::path::Path;

use serde_json::{Value, json};

use crate::session::Session;
use crate::tool::agent::AgentTool;
use crate::tool::{Tool, ToolResult};

async fn execute(params: Value) -> ToolResult {
    AgentTool::new()
        .execute(params)
        .await
        .unwrap_or_else(|error| ToolResult::error(error.to_string()))
}

pub(crate) async fn spawn(
    session: &Session,
    cwd: &Path,
    name: &str,
    instructions: &str,
) -> ToolResult {
    let Some(model) = session.metadata.model.as_deref() else {
        return ToolResult::error("Select a model before spawning a sub-agent");
    };
    execute(json!({
        "action": "spawn", "name": name, "instructions": instructions,
        "model": model, "detach": true, "__ct_session_id": session.id,
        "__ct_parent_workspace": cwd,
    }))
    .await
}

pub(crate) async fn message(parent: &str, name: &str, message: &str) -> ToolResult {
    execute(json!({
        "action": "message", "name": name, "message": message, "detach": true,
        "__ct_session_id": parent,
    }))
    .await
}

pub(crate) async fn kill(parent: &str, name: &str) -> ToolResult {
    execute(json!({ "action": "kill", "name": name, "__ct_session_id": parent })).await
}
