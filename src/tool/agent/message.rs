//! Handle the "message" action.

use super::event_loop;
use super::helpers;
use super::message_result;
use super::store;
use crate::session::SessionEvent;
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use tokio::sync::mpsc;

/// Sends a message to an existing spawned sub-agent and returns its response.
///
/// The handler streams intermediate events, persists the updated session, and
/// formats the final reply as structured JSON.
///
/// # Examples
///
/// ```ignore
/// let result = handle_message(&params).await?;
/// ```
pub(super) async fn handle_message(params: &helpers::Params) -> Result<ToolResult> {
    let name = params
        .name
        .as_ref()
        .context("name required for message")?
        .clone();
    let message = params.message.as_ref().context("message required")?.clone();

    let session = store::get(&name)
        .map(|e| e.session)
        .context(format!("Agent @{name} not found"))?;
    let registry = helpers::get_registry().await?;
    let (tx, mut rx) = mpsc::channel::<SessionEvent>(256);
    let mut session_for_task = session.clone();

    let handle = tokio::spawn(async move {
        session_for_task
            .prompt_with_events(&message, tx, registry)
            .await
            .map(|_| session_for_task)
    });

    let (response, thinking, tools, error, updated_session) =
        event_loop::run(&mut rx, handle).await;

    if let Some(updated) = updated_session {
        store::update_session(&name, updated.clone());
        if let Err(e) = updated.save().await {
            tracing::warn!(agent = %name, error = %e, "Failed to save agent session after message");
        }
    }

    Ok(message_result::build_message_result(
        name, response, thinking, tools, error,
    ))
}
