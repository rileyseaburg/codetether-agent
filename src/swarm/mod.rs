//! Swarm orchestration for parallel sub-agent execution
//!
//! Implements the SubAgent/SubTask paradigm for parallel task execution,
//! similar to Kimi K2.5's Agent Swarm but generalized for the CodeTether ecosystem.
//!
//! Key concepts:
//! - **Orchestrator**: Decomposes complex tasks into parallelizable subtasks
//! - **SubAgent**: A dynamically instantiated agent for executing a subtask
//! - **SubTask**: A unit of work that can be executed in parallel
//! - **Critical Path**: Latency-oriented metric for parallel execution
pub use codetether_swarm_core::{StageStats, SwarmStats};
pub mod actor_status;
pub mod cache;
pub use codetether_swarm_git as collapse_controller;
pub mod control;
pub mod delegation;
pub mod delegation_outcome;
pub mod executor;
pub mod k8s_result;
pub mod kubernetes_executor;
mod live_bus;
pub mod orchestrator;
mod quality_shell;
pub mod rate_limiter;
pub mod remote_subtask;
pub mod result_store;
pub mod speculative;
pub mod subtask;
pub mod token_exhaustion;
pub mod token_truncate;
pub(crate) mod tool_policy;
pub mod validation;
pub(crate) mod worktree_branch;
pub(crate) mod worktree_commit;
pub use cache::{CacheConfig, CacheStats, SwarmCache};
pub use collapse_controller::{
    BranchEvaluation, BranchObservation, BranchRuntimeState, CoherenceScore, CollapseController,
    CollapsePolicy, CollapseTick, KillDecision,
};
pub use control::SwarmControl;
pub use executor::{SwarmExecutor, run_agent_loop};
pub use orchestrator::Orchestrator;
pub use rate_limiter::{AdaptiveRateLimiter, RateLimitInfo, RateLimitStats};
pub use result_store::{ResultStore, ResultStoreContext, SharedResult, SubTaskStoreHandle};
pub use subtask::{SubAgent, SubTask, SubTaskContext, SubTaskResult, SubTaskStatus};

/// Status of an actor in the swarm
pub use actor_status::ActorStatus;
pub use codetether_swarm_core::{
    Actor, DecompositionStrategy, ExecutionMode, Handler, MAX_SUBAGENTS, MAX_TOOL_CALLS,
    SwarmArtifact, SwarmConfig, SwarmMessage, SwarmResult,
};
