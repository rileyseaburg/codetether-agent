//! Ralph loop - the core autonomous execution loop

use super::state_store::RalphStateStore;
use super::types::*;
use super::verification::run_story_verification;
use super::{git_policy, quality_shell};
use crate::bus::AgentBus;
use crate::bus::relay::{ProtocolRelayRuntime, RelayAgentProfile};
use crate::provenance::{
    ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance_blocking,
};
use crate::provider::{ContentPart, Message, Provider, ProviderRegistry, Role};
use crate::session::delegation::DelegationState;
use crate::session::{Session, SessionEvent};
use crate::tool::ToolRegistry;
use crate::worktree::WorktreeManager;
use codetether_ralph_core::events::{RalphEvent, RalphStoryInfo};
use codetether_swarm_core::SwarmEvent;
use futures::stream::{self, StreamExt};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

mod parallel;
mod quality;

/// The main Ralph executor
pub struct RalphLoop {
    state: RalphState,
    provider: Arc<dyn Provider>,
    model: String,
    config: RalphConfig,
    event_tx: Option<mpsc::Sender<RalphEvent>>,
    bus: Option<Arc<AgentBus>>,
    registry: Option<Arc<ProviderRegistry>>,
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
