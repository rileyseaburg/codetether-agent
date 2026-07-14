//! # Local Stage Execution
//!
//! Runs one dependency stage as concurrent local sub-agents with optional Git
//! worktree isolation. `State` tracks launched tasks, worktrees, collapse
//! decisions, and cached results across the stage lifecycle.
//!
//! The parent executor calls `execute`; preparation, launch, collection,
//! integration, and caching are delegated to focused child modules.

mod cache_filter;
mod cache_store;
mod collapse;
mod collapse_audit;
mod collapse_events;
mod collapse_kill;
mod collect;
mod completion;
mod context;
mod create;
mod create_resources;
mod delegation;
mod delegation_outcome;
mod failure;
mod finish_events;
mod integrate;
mod integrate_worktree;
mod job;
mod job_builder;
mod job_run;
mod killed_worktree;
mod launch;
mod prompts;
mod result;
mod result_make;
mod result_success;
mod retry;
mod retry_attempt;
mod retry_log;
mod runner;
mod start_events;
mod state;
mod worktrees;
pub(super) use runner::execute;
