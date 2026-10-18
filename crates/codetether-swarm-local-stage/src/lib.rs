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
mod integrate;
mod integrate_worktree;
mod job;
mod job_builder;
mod killed_worktree;
mod launch;
mod runner;
mod state;
mod worktrees;
pub use runner::execute;
mod host;
pub use host::{Cache, Execution, Host, Providers, Services};
mod worktree_failure;
mod worktree_setup;
