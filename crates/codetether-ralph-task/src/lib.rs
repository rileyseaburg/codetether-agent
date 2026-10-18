//! Owned parallel Ralph story tasks and their quality-repair lifecycle.
//!
//! Construct [Task] with captured stage resources and call [Task::run]. The task
//! retains its permit, event bridge and borrowed execution context through model
//! retries, quality repair and progress recording. [Host] supplies native relay
//! and policy-aware verification while [Host::Agent] binds the shared agent loop.
mod host;
mod parallel_call_direct;
mod parallel_call_mode;
mod parallel_execution_context;
mod parallel_quality_exhausted;
mod parallel_quality_host;
mod parallel_quality_outcome;
mod parallel_quality_repair;
mod parallel_quality_verify;
mod parallel_task;
mod parallel_task_announce;
mod parallel_task_bridge;
mod parallel_task_context;
mod parallel_task_execute;
mod parallel_task_run;
mod repair;
mod result;
use codetether_bus_core::AgentBus;
use codetether_context_policy::delegation::DelegationState;
use codetether_provider_core::Provider;
use codetether_ralph_core::{
    ProgressEntry, QualityChecks, UserStory, events::RalphEvent, quality_repair::QualityProgress,
};
use codetether_swarm_core::SwarmEvent;
use codetether_worktree::{WorktreeInfo, WorktreeManager};
pub use host::Host;
pub use parallel_execution_context::ParallelExecutionContext as Context;
pub use parallel_task::ParallelTask as Task;
pub use result::StoryResult;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc;
use tracing::{info, warn};
