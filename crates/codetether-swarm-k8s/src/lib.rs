//! # Kubernetes Stage Execution
//!
//! Runs swarm subtasks in isolated Kubernetes pods subject to a pod budget.
//! `State` owns pending work, active pod branches, results, and collapse state.
//!
//! The stage runner repeatedly fills available slots, polls completed pods,
//! records results, samples collapse policy, and removes residual pods.

mod cache;
mod cleanup;
mod collapse;
mod collapse_audit;
mod collapse_kill;
mod context;
mod control;
mod events;
mod failure;
mod final_result;
mod finish_events;
mod observation;
mod payload;
mod pod_cleanup;
mod poll;
mod poll_one;
mod record;
mod runner;
mod spawn;
mod spec;
mod state;
mod state_create;
mod timeout;
pub use runner::execute;
mod host;
pub use host::{Cache, Host};
/// Pod health scoring shared with native compatibility callers.
pub mod resource_health;

/// Remote-worker branch observation and probe thread lifecycle.
pub mod probe;
