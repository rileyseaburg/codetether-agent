//! Preparation sequence shared by streaming and non-streaming callers.
use super::{Host, Parts};
use crate::{LoopState, WorkspaceState};
use anyhow::Result;
use std::collections::HashSet;

/// Prepare a prompt turn's model, workspace, lease, and progress state.
/// # Arguments
/// * `host` — Borrowed application services and session state.
/// # Returns
/// Owned state ready to attach to the application's prompt runner.
/// # Errors
/// Returns errors selecting the initial provider/model after notification.
/// # Examples
/// See [Parts] for an initialization example.
pub async fn initialize<H: Host>(host: &mut H) -> Result<Parts<H::Model, H::Router, H::Subagents>> {
    host.thinking().await;
    let model = host.resolve_model()?;
    let cwd = host
        .directory()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let baseline_dirty = host.capture_dirty(&cwd).await;
    let max_steps = host.max_steps();
    let lease_owner = format!("{}:{}", host.session_id(), uuid::Uuid::new_v4());
    Ok(Parts {
        lease_owner,
        model,
        workspace: WorkspaceState {
            cwd,
            baseline_dirty,
            touched: HashSet::new(),
        },
        progress: LoopState::new(max_steps),
        router: host.router(),
        subagents: H::Subagents::default(),
    })
}
