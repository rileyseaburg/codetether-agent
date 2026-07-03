//! Auto-starts the first model turn after a sub-agent is spawned (issue #295).
//!
//! Previously, `spawn` only created a session and stored it — no model call was
//! ever made until the caller explicitly invoked `message`. This left spawned
//! agents permanently idle (`no_activity`).
//!
//! This module sends a kick-off user message and runs the turn either detached
//! (background tokio task) or synchronously.

use super::bus_publish;
use super::event_loop;
use super::execution_state;
use super::helpers;
use super::message_detach;
use super::message_finalize;
use super::store;
use crate::session::SessionEvent;
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use tokio::sync::mpsc;

/// Kick off the first turn for a freshly spawned sub-agent.
///
/// When `detach` is true the turn runs in a background task and returns
/// immediately; otherwise it blocks until the turn completes.
pub(super) async fn kick_off(name: &str, detach: bool) -> Result<ToolResult> {
    let Some(_guard) = execution_state::try_start(name) else {
        return Ok(ToolResult::success(format!(
            "Spawned @{name}. Agent will start once its current turn finishes."
        )));
    };
    let session = store::get(name)
        .map(|e| e.session)
        .context(format!("Agent @{name} vanished after spawn"))?;
    let registry = helpers::get_registry().await?;
    bus_publish::announce_working(name, "starting first turn");
    let (tx, mut rx) = mpsc::channel::<SessionEvent>(256);
    let mut session_for_task = session.clone();
    if session_for_task.bus.is_none() {
        session_for_task.bus = crate::bus::global();
    }
    let handle = tokio::spawn(async move {
        session_for_task
            .prompt_with_events(KICKOFF_MSG, tx, registry)
            .await
            .map(|_| session_for_task)
    });
    if detach {
        return Ok(message_detach::dispatch(
            name.to_string(),
            _guard,
            rx,
            handle,
        ));
    }
    let r = event_loop::run(&mut rx, handle).await;
    Ok(message_finalize::finalize(name.to_string(), r.0, r.1, r.2, r.3, r.4).await)
}

const KICKOFF_MSG: &str = "Begin working on your assigned task now.";
