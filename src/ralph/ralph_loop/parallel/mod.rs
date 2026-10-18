//! Parallel story execution: task preparation, worktree isolation, quality and collection.
//!
//! The driver supplies the shared scheduler with application stage callbacks.
//! Shared tasks retain their owned resources until collection and stage finalization;
//! merge callers borrow the result context without taking ownership.
use super::*;
mod driver;
mod host;
mod parallel_apply;
mod parallel_collect;
mod parallel_create_worktree;
mod parallel_failure;
mod parallel_finish_stage;
mod parallel_plan;
pub(super) mod parallel_result_types;
mod parallel_spawn;
mod parallel_stage;
mod parallel_stage_state;
mod parallel_task;
mod parallel_task_create;
mod parallel_worktrees;
mod scheduling_parallel;
