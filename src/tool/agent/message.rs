//! Handle the "message" action.

use super::store;
use super::helpers;
use super::event_loop;
use crate::session::SessionEvent;
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use serde_json::json;
use tokio::sync::mpsc;

pub(super) async fn handle_message(params: &helpers::Params) -> Result<ToolResult> {
    let name = params.name.as_ref().context("name required for message")?.clone();
    let message = params.message.as_ref().context("message required")?.clone();

    let session = store::get(&name).map(|e| e.session).context(format!("Agent @{name} not found"))?;
    let registry = helpers::get_registry().await?;
    let (tx, mut rx) = mpsc::channel::<SessionEvent>(256);
    let mut session_for_task = session.clone();

    let handle = tokio::spawn(async move {
        session_for_task
            .prompt_with_events(&message, tx, registry)
            .await
            .map(|_| session_for_task)
    });

    let (response, thinking, tools, error, updated_session) = event_loop::run(&mut rx, handle).await;

    if let Some(updated) = updated_session {
        store::update_session(&name, updated.clone());
        if let Err(e) = updated.save().await {
            tracing::warn!(agent = %name, error = %e, "Failed to save agent session after message");
        }
    }

    let mut output = json!({ "agent": name, "response": response });
    if !thinking.is_empty() {
        output["thinking"] = json!(thinking);
    }
    if !tools.is_empty() {
        output["tool_calls"] = json!(tools);
    }
    if let Some(err) = error {
        if response.is_empty() {
            return Ok(ToolResult::error(format!("Agent @{name} failed: {err}")));
        }
        output["warning"] = json!(err);
    }

    Ok(ToolResult::success(serde_json::to_string_pretty(&output).unwrap_or(response)))
}
