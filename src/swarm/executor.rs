//! # Swarm Execution
//!
//! Coordinates decomposed swarm tasks across local agents or Kubernetes pods.
//! [`SwarmExecutor`] owns runtime dependencies, while focused child modules
//! handle orchestration, stage backends, agent turns, tools, and integration.
//!
//! ## Usage
//!
//! ```rust
//! use codetether_agent::swarm::{SwarmConfig, SwarmExecutor};
//! let executor = SwarmExecutor::new(SwarmConfig::default());
//! assert!(executor.control().is_none());
//! ```

macro_rules! executor_module {
    ($visibility:vis $name:ident => $path:literal) => {
        #[path = $path]
        $visibility mod $name;
    };
}

executor_module!(accessors => "executor_parts/accessors.rs");
#[path = "executor_parts/agent_loop/mod.rs"]
mod agent_loop;
executor_module!(agent_loop_large_result => "agent_loop_large_result.rs");
executor_module!(backoff => "executor_parts/backoff.rs");
executor_module!(builder => "executor_parts/builder.rs");
executor_module!(bus_publish => "bus_publish.rs");
executor_module!(cache_api => "executor_parts/cache_api.rs");
executor_module!(cache_result => "cache_result.rs");
executor_module!(change_expectations => "change_expectations.rs");
executor_module!(configure => "executor_parts/configure.rs");
executor_module!(configure_events => "executor_parts/configure_events.rs");
executor_module!(dependency_failure => "dependency_failure.rs");
executor_module!(dependency_failure_events => "dependency_failure_events.rs");
executor_module!(execute_api => "executor_parts/execute_api.rs");
executor_module!(executor_trace => "executor_trace.rs");
executor_module!(exit => "executor_parts/exit.rs");
#[path = "executor_parts/k8s_stage/mod.rs"]
mod k8s_stage;
executor_module!(large_result => "executor_parts/large_result.rs");
executor_module!(large_result_format => "executor_parts/large_result_format.rs");
#[path = "executor_parts/local_stage/mod.rs"]
mod local_stage;
executor_module!(pub loop_step => "loop_step.rs");
executor_module!(merge_outcome => "merge_outcome.rs");
executor_module!(path_guard => "path_guard/mod.rs");
executor_module!(resource_health => "executor_parts/resource_health.rs");
#[path = "executor_parts/run/mod.rs"]
mod run;
executor_module!(stage_dispatch => "executor_parts/stage_dispatch.rs");
executor_module!(types => "executor_parts/types.rs");
executor_module!(pub(crate) workspace_registry => "workspace_registry/mod.rs");
executor_module!(worktree_failure => "worktree_failure.rs");
executor_module!(worktree_setup => "worktree_setup.rs");

pub use super::SwarmMessage;
use super::subtask::SubTaskResult;
pub(super) use super::{kubernetes_executor, tool_policy};
pub use agent_loop::run_agent_loop;
pub use builder::SwarmExecutorBuilder;
pub use exit::AgentLoopExit;
pub use types::SwarmExecutor;
