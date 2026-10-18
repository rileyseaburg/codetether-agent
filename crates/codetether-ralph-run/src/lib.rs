//! Top-level Ralph run lifecycle with native execution callbacks.
//!
//! [run] starts and records a run, chooses its scheduling mode, classifies the
//! terminal state, cleans worktrees and reports completion. Native [Host]
//! implementations retain providers, state ownership and branch-change policy.
mod host;
mod lifecycle_cleanup;
mod lifecycle_finish;
mod lifecycle_record;
mod lifecycle_start;
mod run;
use codetether_ralph_core::{
    RalphConfig, RalphState, RalphStatus, events::RalphEvent, state_store::RalphStateStore,
};
use codetether_worktree::WorktreeManager;
pub use host::{Execution, Host, State};
use lifecycle_cleanup::cleanup_run_worktrees;
use lifecycle_finish::finish_run;
use lifecycle_record::create_initial_run_record;
use lifecycle_start::start_run;
pub use run::run;
use std::sync::Arc;
use tracing::{info, warn};
