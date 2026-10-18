//! Ralph state ownership and integration of the extracted execution services.
//!
//! Construct [RalphLoop] with a native [Host] binding, attach optional providers,
//! bus and persistence services, then run sequential or parallel stories.
//! Scheduling, model execution, merge, Git and lifecycle policy have dedicated crates.
/// Runtime service composition using the shared application registries and agent.
pub mod application;
mod cli;
mod host;
use codetether_bus_core::AgentBus;
use codetether_context_policy::delegation::DelegationState;
use codetether_provider_core::Provider;
use codetether_ralph_core::{events::RalphEvent, state_store::RalphStateStore, *};
use codetether_worktree::WorktreeManager;
pub use host::Host;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc;
use tracing::info;
mod parallel;
mod quality;

/// Owns a Ralph run and connects its dedicated execution services.
/// # Examples
/// ```rust,no_run
/// # async fn example<H: codetether_ralph_runtime::Host>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) -> anyhow::Result<()> {
/// let run = codetether_ralph_runtime::RalphLoop::<H>::new("prd.json".into(), provider, "model".into(), Default::default()).await?;
/// assert_eq!(run.status().status, codetether_ralph_core::RalphStatus::Pending);
/// # Ok(()) }
/// ```
pub struct RalphLoop<H: Host> {
    state: RalphState,
    provider: Arc<dyn Provider>,
    model: String,
    config: RalphConfig,
    event_tx: Option<mpsc::Sender<RalphEvent>>,
    bus: Option<Arc<AgentBus>>,
    registry: Option<Arc<H::Registry>>,
    store: Option<Arc<dyn RalphStateStore>>,
    /// Optional CADMAS-CTX delegation state for LCB relay provider selection
    delegation: Option<DelegationState>,
    run_id: String,
}

/// Create a sample PRD template
pub fn create_prd_template(project: &str, feature: &str) -> Prd {
    codetether_ralph_core::create_prd_template(project, feature)
}

mod relay;

mod sequential;

mod workspace;

mod lifecycle;

mod model;

mod reporting;
