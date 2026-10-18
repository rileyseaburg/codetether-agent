//! Shared swarm status, metrics, rate-limit policy and event contracts.
//!
//! Execution and rendering consume these contracts without owning their
//! definitions. Metrics retain their inherent calculation methods.
mod methods;
mod types;
pub use types::*;
pub mod rate_limiter;
mod task_kind;
mod task_methods;
mod transient_error;
pub use transient_error::is_transient_error;

/// Disk-backed task-result caching.
pub mod cache;

pub mod config;
pub use config::{
    DecompositionStrategy, ExecutionMode, MAX_SUBAGENTS, MAX_TOOL_CALLS, SwarmConfig,
};
pub mod outcome;
pub use outcome::{SwarmArtifact, SwarmResult};
pub mod protocol;
pub use protocol::{Actor, ActorStatus, Handler, SwarmMessage};

/// Decomposition parsing, dependency validation and prompt construction.
pub mod planning;

/// Task and agent coordination state.
pub mod coordinator;
pub use protocol::{OrchestratorMessage, SubAgentMessage};

/// Shared result storage, subscriptions and task-scoped handles.
pub mod result_store;

/// Backend-independent branch-collapse policy.
pub mod collapse;

/// Pure pre-flight validation and report formatting.
pub mod validation;

/// Context budgets, message truncation and exhaustion reporting.
pub mod context;

/// Run aggregation, completion statistics and failed-dependency gating.
pub mod run_policy;

/// Shared dependency-stage execution coordinator and host interface.
pub mod stage_runtime;

/// Retry policy and termination contracts for sub-agent attempts.
pub mod agent_retry;

/// Local agent-attempt result construction.
pub mod agent_result;
/// Delegated deliverable status and verification evidence policy.
pub mod deliverable;

/// Remote payload, log and result protocol.
pub mod remote_protocol;

/// Shared sub-agent provider-turn driver and host operations.
pub mod agent_loop;

/// Delegated prompt composition and sourced constraint policy.
pub mod prompt_policy;

/// Worker lifecycle output formatting.
pub mod worker_output;
/// Worker completion result text.
pub mod worker_result;
/// Worker strategy-name aliases.
pub mod worker_strategy;

/// Historical model defaults shared by planning and direct swarm workers.
pub mod model_defaults;

/// Autonomous tool restrictions and advertised read-only definitions.
pub mod tool_policy;

/// Fluent configuration builder for a host-provided executor.
pub mod builder;
