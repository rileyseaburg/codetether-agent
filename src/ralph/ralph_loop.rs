//! Ralph loop - the core autonomous execution loop

use super::state_store::{RalphRunState, RalphStateStore, StoryResultEntry};
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
use crate::swarm::{executor::AgentLoopExit, run_agent_loop};
use crate::tool::ToolRegistry;
use crate::worktree::WorktreeManager;
use codetether_ralph_core::events::{RalphEvent, RalphStoryInfo, RalphStoryStatus};
use codetether_swarm_core::SwarmEvent;
use futures::stream::{self, StreamExt};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

mod bus_collect;
mod bus_publish;
mod configure;
mod conflict_complete;
mod conflict_execute;
mod conflict_invoke;
mod conflict_merge;
mod conflict_result;
mod conflict_verify;
mod conflicts;
mod construct;
mod event_bridge;
mod events;
mod git_checkout;
mod git_commit;
mod git_commit_dir;
mod lifecycle_cleanup;
mod lifecycle_finish;
mod lifecycle_record;
mod lifecycle_start;
mod merge_dispatch;
mod merge_nonconflict;
mod merge_story;
mod merge_success;
mod merge_verify;
mod merge_without_worktree;
mod model_direct;
mod model_exit;
mod model_static;
mod parallel;
mod parallel_apply;
mod parallel_call_direct;
mod parallel_call_mode;
mod parallel_collect;
mod parallel_create_worktree;
mod parallel_execution_context;
mod parallel_failure;
mod parallel_finish_stage;
mod parallel_plan;
mod parallel_quality_exhausted;
mod parallel_quality_loop;
mod parallel_quality_outcome;
mod parallel_quality_repair;
mod parallel_quality_state;
mod parallel_quality_verify;
mod parallel_repair;
mod parallel_result;
mod parallel_result_types;
mod parallel_spawn;
mod parallel_stage;
mod parallel_stage_state;
mod parallel_task;
mod parallel_task_announce;
mod parallel_task_bridge;
mod parallel_task_context;
mod parallel_task_create;
mod parallel_task_execute;
mod parallel_task_run;
mod parallel_worktrees;
mod persistence;
mod progress;
mod prompts;
mod provenance;
mod quality;
mod relay_announce;
mod relay_coordinator;
mod relay_events;
mod relay_finish;
mod relay_prepare;
mod relay_profiles;
mod relay_progress;
mod relay_rounds;
mod relay_sender;
mod relay_session;
mod relay_text;
mod relay_turn;
mod relay_types;
mod seq_error;
mod seq_failure;
mod seq_iteration;
mod seq_prompt;
mod seq_response;
mod seq_success;
mod seq_unchecked;
mod sequential;
mod status;

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

impl RalphLoop {
    /// Run the Ralph loop until completion or max iterations
    pub async fn run(&mut self) -> anyhow::Result<RalphState> {
        self.start_run()?;
        // Choose execution mode
        if self.config.parallel_enabled {
            self.run_parallel().await?;
        } else {
            self.run_sequential().await?;
        }

        codetether_ralph_core::lifecycle::finalize_status(&mut self.state);
        self.cleanup_run_worktrees().await;
        self.finish_run()
    }
}

/// Create a sample PRD template
pub fn create_prd_template(project: &str, feature: &str) -> Prd {
    codetether_ralph_core::create_prd_template(project, feature)
}
