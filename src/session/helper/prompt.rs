//! Non-streaming entry point for the shared agentic prompt loop.
//!
//! This adapter loads the provider registry and invokes the same engine used
//! by the TUI without attaching an event channel.

use anyhow::Result;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionResult};

/// Executes a plain-text prompt without streaming session events.
///
/// # Errors
///
/// Returns an error when provider setup, a model call, tool execution, or
/// session persistence fails.
pub(crate) async fn run_prompt(session: &mut Session, message: &str) -> Result<SessionResult> {
    let registry = Arc::new(ProviderRegistry::from_vault().await?);
    crate::session::step_limit::mark_budget_active();
    let mut runner = super::prompt_loop::initialize(session, None, registry).await?;
    runner.accept(message, Vec::new()).await?;
    super::prompt_loop::run(&mut runner).await
}
