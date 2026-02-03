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

pub mod executor;
pub mod orchestrator;
pub mod subtask;

pub use executor::SwarmExecutor;
pub use orchestrator::Orchestrator;
pub use subtask::{SubAgent, SubTask, SubTaskContext, SubTaskResult, SubTaskStatus};

use async_trait::async_trait;
use anyhow::Result;

/// Actor trait for swarm participants
/// 
/// An Actor is an entity that can participate in the swarm by receiving
/// and processing messages. This is the base trait for all swarm participants.
#[async_trait]
pub trait Actor: Send + Sync {
    /// Get the unique identifier for this actor
    fn actor_id(&self) -> &str;
    
    /// Get the actor's current status
    fn actor_status(&self) -> ActorStatus;
    
    /// Initialize the actor for swarm participation
    async fn initialize(&mut self) -> Result<()>;
    
    /// Shutdown the actor gracefully
    async fn shutdown(&mut self) -> Result<()>;
}

/// Handler trait for processing messages in the swarm
/// 
/// A Handler can receive and process messages of a specific type.
/// This enables actors to respond to different message types.
#[async_trait]
pub trait Handler<M>: Actor {
    /// The response type for this handler
    type Response: Send + Sync;
    
    /// Handle a message and return a response
    async fn handle(&mut self, message: M) -> Result<Self::Response>;
}

/// Status of an actor in the swarm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorStatus {
    /// Actor is initializing
    Initializing,
    /// Actor is ready to process messages
    Ready,
    /// Actor is currently processing
    Busy,
    /// Actor is paused
    Paused,
    /// Actor is shutting down
    ShuttingDown,
    /// Actor has stopped
    Stopped,
}

impl std::fmt::Display for ActorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorStatus::Initializing => write!(f, "initializing"),
            ActorStatus::Ready => write!(f, "ready"),
            ActorStatus::Busy => write!(f, "busy"),
            ActorStatus::Paused => write!(f, "paused"),
            ActorStatus::ShuttingDown => write!(f, "shutting_down"),
            ActorStatus::Stopped => write!(f, "stopped"),
        }
    }
}

/// Message types for swarm coordination
#[derive(Debug, Clone)]
pub enum SwarmMessage {
    /// Execute a task
    ExecuteTask { task_id: String, instruction: String },
    /// Report progress
    Progress { task_id: String, progress: f32, message: String },
    /// Task completed
    TaskCompleted { task_id: String, result: String },
    /// Task failed
    TaskFailed { task_id: String, error: String },
    /// Request tool execution
    ToolRequest { tool_id: String, arguments: serde_json::Value },
    /// Tool response
    ToolResponse { tool_id: String, result: crate::tool::ToolResult },
}

use serde::{Deserialize, Serialize};

/// Maximum number of concurrent sub-agents
pub const MAX_SUBAGENTS: usize = 100;

/// Maximum total tool calls across all sub-agents
pub const MAX_TOOL_CALLS: usize = 1500;

/// Swarm execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmConfig {
    /// Maximum number of concurrent sub-agents
    pub max_subagents: usize,
    
    /// Maximum tool calls per sub-agent
    pub max_steps_per_subagent: usize,
    
    /// Maximum total tool calls across all sub-agents
    pub max_total_steps: usize,
    
    /// Timeout for individual sub-agent execution (seconds)
    pub subagent_timeout_secs: u64,
    
    /// Whether to enable parallel execution
    pub parallel_enabled: bool,
    
    /// Critical path optimization threshold
    pub critical_path_threshold: usize,
    
    /// Model to use for sub-agents (provider/model format)
    pub model: Option<String>,
    
    /// Max concurrent API requests (rate limiting)
    pub max_concurrent_requests: usize,
    
    /// Delay between API calls in ms (rate limiting)
    pub request_delay_ms: u64,
    
    /// Enable worktree isolation for sub-agents
    pub worktree_enabled: bool,
    
    /// Automatically merge worktree changes on success
    pub worktree_auto_merge: bool,
    
    /// Working directory for worktree creation
    pub working_dir: Option<String>,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            max_subagents: MAX_SUBAGENTS,
            max_steps_per_subagent: 100,
            max_total_steps: MAX_TOOL_CALLS,
            subagent_timeout_secs: 600,  // 10 minutes for complex tasks
            parallel_enabled: true,
            critical_path_threshold: 10,
            model: None,
            max_concurrent_requests: 3,  // V1 tier allows 3 concurrent
            request_delay_ms: 1000,      // V1 tier: 60 RPM, 3 concurrent = fast
            worktree_enabled: true,      // Enable worktree isolation by default
            worktree_auto_merge: true,   // Auto-merge on success
            working_dir: None,
        }
    }
}

/// Swarm execution statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmStats {
    /// Total number of sub-agents spawned
    pub subagents_spawned: usize,
    
    /// Number of sub-agents that completed successfully
    pub subagents_completed: usize,
    
    /// Number of sub-agents that failed
    pub subagents_failed: usize,
    
    /// Total tool calls across all sub-agents
    pub total_tool_calls: usize,
    
    /// Critical path length (longest chain of dependent steps)
    pub critical_path_length: usize,
    
    /// Wall-clock execution time (milliseconds)
    pub execution_time_ms: u64,
    
    /// Estimated sequential time (milliseconds)
    pub sequential_time_estimate_ms: u64,
    
    /// Parallelization speedup factor
    pub speedup_factor: f64,
    
    /// Per-stage statistics
    pub stages: Vec<StageStats>,
}

/// Statistics for a single execution stage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StageStats {
    /// Stage index
    pub stage: usize,
    
    /// Number of sub-agents in this stage
    pub subagent_count: usize,
    
    /// Maximum steps in this stage (critical path contribution)
    pub max_steps: usize,
    
    /// Total steps across all sub-agents in this stage
    pub total_steps: usize,
    
    /// Execution time for this stage (milliseconds)
    pub execution_time_ms: u64,
}

impl SwarmStats {
    /// Calculate the speedup factor
    pub fn calculate_speedup(&mut self) {
        if self.execution_time_ms > 0 {
            self.speedup_factor = self.sequential_time_estimate_ms as f64 / self.execution_time_ms as f64;
        }
    }
    
    /// Calculate critical path from stages
    pub fn calculate_critical_path(&mut self) {
        self.critical_path_length = self.stages.iter().map(|s| s.max_steps).sum();
    }
}

/// Decomposition strategy for breaking down tasks
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DecompositionStrategy {
    /// Let the AI decide how to decompose
    Automatic,
    
    /// Decompose by domain/specialty
    ByDomain,
    
    /// Decompose by data partition
    ByData,
    
    /// Decompose by workflow stage
    ByStage,
    
    /// Single agent (no decomposition)
    None,
}

impl Default for DecompositionStrategy {
    fn default() -> Self {
        Self::Automatic
    }
}

/// Result of swarm execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmResult {
    /// Overall success status
    pub success: bool,
    
    /// Aggregated result from all sub-agents
    pub result: String,
    
    /// Individual sub-task results
    pub subtask_results: Vec<SubTaskResult>,
    
    /// Execution statistics
    pub stats: SwarmStats,
    
    /// Any artifacts produced
    pub artifacts: Vec<SwarmArtifact>,
    
    /// Error message if failed
    pub error: Option<String>,
}

/// An artifact produced by the swarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmArtifact {
    /// Artifact type
    pub artifact_type: String,
    
    /// Name/identifier
    pub name: String,
    
    /// Content or path
    pub content: String,
    
    /// Which sub-agent produced it
    pub source_subagent: Option<String>,
    
    /// MIME type
    pub mime_type: Option<String>,
}
