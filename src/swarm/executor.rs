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

#[path = "executor_parts/agent_loop/mod.rs"]
mod agent_loop;
executor_module!(dependency_failure => "dependency_failure.rs");
executor_module!(exit => "executor_parts/exit.rs");
executor_module!(pub loop_step => "loop_step.rs");
executor_module!(path_guard => "path_guard/mod.rs");
executor_module!(types => "executor_parts/types.rs");
executor_module!(application_host => "executor_parts/application_host.rs");
executor_module!(pub(crate) workspace_registry => "workspace_registry/mod.rs");

pub use super::SwarmMessage;

use super::subtask::SubTaskResult;
pub(super) use super::{kubernetes_executor, tool_policy};
pub use agent_loop::run_agent_loop;
/// Fluent builder using the application swarm executor.
pub type SwarmExecutorBuilder = codetether_swarm_core::builder::SwarmExecutorBuilder<SwarmExecutor>;
pub use exit::AgentLoopExit;
pub use types::SwarmExecutor;
