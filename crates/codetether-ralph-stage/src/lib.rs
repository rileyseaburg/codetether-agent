//! Parallel Ralph stage planning, task lifetime management and collection.
//!
//! Implement [Host] to lend the live run inputs and merge successful stories.
//! [run] coordinates dependency stages using the shared scheduler; each stage
//! pre-creates bounded worktrees, spawns owned tasks, collects them in order,
//! publishes progress and checkpoints the PRD before releasing its resources.
mod host;
mod inputs;
mod parallel_apply;
mod parallel_collect;
mod parallel_create_worktree;
mod parallel_failure;
mod parallel_finish_stage;
mod parallel_plan;
mod parallel_spawn;
mod parallel_stage;
mod parallel_stage_state;
mod parallel_task_create;
mod parallel_worktrees;
mod reporting;
mod runtime;
use codetether_bus_core::AgentBus;
use codetether_context_policy::delegation::DelegationState;
use codetether_provider_core::Provider;
use codetether_ralph_core::{
    RalphConfig, RalphState, RalphStatus, UserStory, events::RalphEvent,
    state_store::RalphStateStore,
};
use codetether_ralph_task::{Host as TaskHost, StoryResult as ParallelStoryResult, Task};
use codetether_worktree::{WorktreeInfo, WorktreeManager};
use futures::stream::{self, StreamExt};
pub use host::Host;
pub use inputs::Inputs;
use parallel_apply::apply_parallel_result;
use parallel_collect::collect_parallel_results;
use parallel_create_worktree::create_story_worktree;
use parallel_failure::report_failed_parallel_story;
use parallel_finish_stage::finish_parallel_stage;
pub use parallel_plan::{parallel_stages, parallel_worktree_manager};
use parallel_spawn::spawn_parallel_stage;
pub use parallel_stage::begin_parallel_stage;
use parallel_stage_state::ParallelStage;
use parallel_task_create::prepare_parallel_task;
use parallel_worktrees::precreate_stage_worktrees;
use reporting::{
    bus_collect_learnings, bus_publish_story_result, record_story_result, try_send_event,
};
pub use runtime::{execute_stage, run};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc;
use tracing::{info, warn};
