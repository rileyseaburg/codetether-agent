//! Streaming entry point for the shared agentic prompt loop.
//!
//! This module adapts the shared prompt engine to a caller-provided event
//! channel and optional image attachments. Provider calls, retries, tool
//! execution, validation, and persistence remain owned by `prompt_loop`.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionEvent, SessionResult};

#[path = "prompt_subagent_watch.rs"]
pub(super) mod subagent_watch;

pub(super) use super::tool_exec::execute_tool;

/// Executes a prompt and streams lifecycle events to `event_tx`.
///
/// # Errors
///
/// Returns an error when provider setup, a model call, tool execution, or
/// session persistence fails.
pub(crate) async fn run_prompt_with_events(
    session: &mut Session,
    message: &str,
    images: Vec<ImageAttachment>,
    event_tx: mpsc::Sender<SessionEvent>,
    registry: Arc<ProviderRegistry>,
) -> Result<SessionResult> {
    crate::session::step_limit::mark_budget_active();
    let mut runner = super::prompt_loop::initialize(session, Some(event_tx), registry).await?;
    runner.accept(message, images).await?;
    super::prompt_loop::run(&mut runner).await
}
