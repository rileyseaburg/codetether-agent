//! Construction of shared prompt-loop state.

use anyhow::Result;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::mpsc;

use super::{
    Runner,
    progress::{LoopState, WorkspaceState},
};
use crate::provider::ProviderRegistry;
use crate::session::{DEFAULT_MAX_STEPS, Session, SessionEvent};

/// Initializes shared loop state for a session and optional event channel.
///
/// # Errors
///
/// Returns an error when provider selection or workspace inspection fails.
pub(crate) async fn initialize<'a>(
    session: &'a mut Session,
    events: Option<mpsc::Sender<SessionEvent>>,
    registry: Arc<ProviderRegistry>,
) -> Result<Runner<'a>> {
    if let Some(tx) = &events {
        let _ = tx.send(SessionEvent::Thinking).await;
    }
    session.resolve_subcall_provider(&registry);
    let model = super::setup_model::resolve(session, &registry)?;
    let cwd = session
        .metadata
        .directory
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let baseline_dirty = super::super::validation::capture_git_dirty_files(&cwd).await;
    let max_steps = session.max_steps.unwrap_or(DEFAULT_MAX_STEPS);
    Ok(Runner {
        session,
        events,
        registry,
        model,
        workspace: WorkspaceState {
            cwd,
            baseline_dirty,
            touched: HashSet::new(),
        },
        progress: LoopState::new(max_steps),
        router: super::setup_support::router(),
        subagents: super::super::prompt_events::subagent_watch::SubAgentWatch::default(),
    })
}
