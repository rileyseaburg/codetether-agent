//! Owning-session wrapper over context-window enforcement.

use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use tokio::sync::mpsc;

use crate::provider::ToolDefinition;
use crate::session::{Session, SessionEvent};

use super::context::CompressContext;
use super::enforce::enforce_on_messages;

/// Back-compat wrapper over [`enforce_on_messages`] that operates on an
/// owning [`Session`]. Bumps [`Session::updated_at`] when the buffer was
/// rewritten.
///
/// Prefer [`enforce_on_messages`] for new code — it lets you run
/// context-window enforcement on a history clone without mutating the
/// canonical [`Session::messages`] buffer (the Phase A history/context
/// split).
///
/// # Errors
///
/// Propagates any error from the underlying core.
pub async fn enforce_context_window(
    session: &mut Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    let ctx = CompressContext::from_session(session);
    let before_len = session.messages.len();
    enforce_on_messages(
        &mut session.messages,
        &ctx,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
    )
    .await?;
    if session.messages.len() != before_len {
        session.updated_at = Utc::now();
    }
    Ok(())
}
