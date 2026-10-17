//! Parallel story execution: task preparation, worktree isolation, quality and collection.
//!
//! The driver supplies the shared scheduler with application stage callbacks.
//! Task-owned resources remain here until collection and stage finalization;
//! merge callers borrow the result context without taking ownership.
use super::*;
mod driver;
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
mod parallel_quality_host;
mod parallel_quality_outcome;
mod parallel_quality_repair;
mod parallel_quality_state;
mod parallel_quality_verify;
mod parallel_repair;
mod parallel_result;
pub(super) mod parallel_result_types;
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
mod scheduling_parallel;
