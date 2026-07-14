//! Handle the "message" action.

use super::event_loop;
use super::execution_state;
use super::helpers;
use super::message_finalize;
use crate::session::SessionEvent;
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use tokio::sync::mpsc;

#[path = "message/session.rs"]
mod task_session;

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
    let Some(_run_guard) = execution_state::try_start(&name) else {
        return Ok(ToolResult::error(format!(
            "Agent @{name} is busy processing another message; retry shortly"
        )));
    };
    let mut session_for_task = task_session::load(&name, params).await?;
    let registry = helpers::get_registry().await?;
    super::bus_publish::announce_working(&name, "processing message");
    let (tx, mut rx) = mpsc::channel::<SessionEvent>(256);
    let handle = tokio::spawn(async move {
        session_for_task
            .prompt_with_events(&message, tx, registry)
            .await
            .map(|_| session_for_task)
    });
    execution_state::register(&name, &handle);

    if params.detach_or_default() {
        return Ok(super::message_detach::dispatch(
            name, _run_guard, rx, handle,
        ));
    }

    let (response, thinking, tools, error, updated_session) =
        event_loop::run(&mut rx, handle).await;

    Ok(message_finalize::finalize(name, response, thinking, tools, error, updated_session).await)
}
