//! SubTask and SubAgent definitions
//!
//! A SubTask is a unit of work that can be executed in parallel.
//! A SubAgent is a dynamically instantiated agent for executing a subtask.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A sub-task that can be executed by a sub-agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTask {
    /// Unique identifier
    pub id: String,

    /// Human-readable name/description
    pub name: String,

    /// The task instruction for the sub-agent
    pub instruction: String,

    /// Specialty/domain for this subtask
    pub specialty: Option<String>,

    /// Dependencies on other subtasks (by ID)
    pub dependencies: Vec<String>,

    /// Priority (higher = more important)
    pub priority: i32,

    /// Maximum steps allowed for this subtask
    pub max_steps: usize,

    /// Input context from parent or dependencies
    pub context: SubTaskContext,

    /// Current status
    pub status: SubTaskStatus,

    /// Assigned sub-agent ID
    pub assigned_agent: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Completion timestamp
    pub completed_at: Option<DateTime<Utc>>,

    /// Stage in the execution plan (0 = can run immediately)
    pub stage: usize,
}

impl SubTask {
    /// Create a new subtask
    pub fn new(name: impl Into<String>, instruction: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            instruction: instruction.into(),
            specialty: None,
            dependencies: Vec::new(),
            priority: 0,
            max_steps: 100,
            context: SubTaskContext::default(),
            status: SubTaskStatus::Pending,
            assigned_agent: None,
            created_at: Utc::now(),
            completed_at: None,
            stage: 0,
        }
    }

    /// Add a specialty
    pub fn with_specialty(mut self, specialty: impl Into<String>) -> Self {
        self.specialty = Some(specialty.into());
        self
    }

    /// Add dependencies
    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = deps;
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set max steps
    pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        self
    }

    /// Add context
    pub fn with_context(mut self, context: SubTaskContext) -> Self {
        self.context = context;
        self
    }

    /// Check if this subtask can run (all dependencies complete)
    pub fn can_run(&self, completed: &[String]) -> bool {
        self.dependencies.iter().all(|dep| completed.contains(dep))
    }

    /// Mark as running
    pub fn start(&mut self, agent_id: &str) {
        self.status = SubTaskStatus::Running;
        self.assigned_agent = Some(agent_id.to_string());
    }

    /// Mark as completed
    pub fn complete(&mut self, success: bool) {
        self.status = if success {
            SubTaskStatus::Completed
        } else {
            SubTaskStatus::Failed
        };
        self.completed_at = Some(Utc::now());
    }
}

/// Context passed to a subtask
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubTaskContext {
    /// Parent task information
    pub parent_task: Option<String>,

    /// Results from dependency subtasks
    pub dependency_results: HashMap<String, String>,

    /// Shared files or resources
    pub shared_resources: Vec<String>,

    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Status of a subtask
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubTaskStatus {
    /// Waiting to be scheduled
    Pending,

    /// Waiting for dependencies
    Blocked,

    /// Currently executing
    Running,

    /// Successfully completed
    Completed,

    /// Failed with error
    Failed,

    /// Cancelled by orchestrator
    Cancelled,

    /// Timed out
    TimedOut,
}

/// A sub-agent executing a subtask
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgent {
    /// Unique identifier
    pub id: String,

    /// Display name
    pub name: String,

    /// Specialty/role (e.g., "AI Researcher", "Code Writer", "Fact Checker")
    pub specialty: String,

    /// The subtask this agent is working on
    pub subtask_id: String,

    /// Current status
    pub status: SubAgentStatus,

    /// Number of steps taken
    pub steps: usize,

    /// Tool calls made
    pub tool_calls: Vec<ToolCallRecord>,

    /// Accumulated output
    pub output: String,

    /// Model being used
    pub model: String,

    /// Provider
    pub provider: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last activity timestamp
    pub last_active: DateTime<Utc>,
}

impl SubAgent {
    /// Create a new sub-agent for a subtask
    pub fn new(
        name: impl Into<String>,
        specialty: impl Into<String>,
        subtask_id: impl Into<String>,
        model: impl Into<String>,
        provider: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            specialty: specialty.into(),
            subtask_id: subtask_id.into(),
            status: SubAgentStatus::Initializing,
            steps: 0,
            tool_calls: Vec::new(),
            output: String::new(),
            model: model.into(),
            provider: provider.into(),
            created_at: now,
            last_active: now,
        }
    }

    /// Record a tool call
    pub fn record_tool_call(&mut self, name: &str, success: bool) {
        self.tool_calls.push(ToolCallRecord {
            name: name.to_string(),
            timestamp: Utc::now(),
            success,
        });
        self.steps += 1;
        self.last_active = Utc::now();
    }

    /// Append to output
    pub fn append_output(&mut self, text: &str) {
        self.output.push_str(text);
        self.last_active = Utc::now();
    }

    /// Set status
    pub fn set_status(&mut self, status: SubAgentStatus) {
        self.status = status;
        self.last_active = Utc::now();
    }
}

/// Status of a sub-agent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubAgentStatus {
    /// Being initialized
    Initializing,

    /// Actively working
    Working,

    /// Waiting for resource/dependency
    Waiting,

    /// Successfully completed
    Completed,

    /// Failed
    Failed,

    /// Terminated by orchestrator
    Terminated,
}

/// Record of a tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRecord {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
}

/// Determine whether an error message represents a transient failure that
/// is safe to retry (network issues, rate limiting, timeouts).
///
/// Returns `true` if the error is likely transient and a retry may succeed.
pub fn is_transient_error(error: &str) -> bool {
    let lower = error.to_ascii_lowercase();
    // Rate limiting
    if lower.contains("rate limit")
        || lower.contains("ratelimit")
        || lower.contains("too many requests")
        || lower.contains("429")
    {
        return true;
    }
    // Timeout / connection issues
    if lower.contains("timeout")
        || lower.contains("timed out")
        || lower.contains("connection reset")
        || lower.contains("connection refused")
        || lower.contains("broken pipe")
        || lower.contains("network")
        || lower.contains("socket")
        || lower.contains("io error")
    {
        return true;
    }
    // Transient server errors
    if lower.contains("503")
        || lower.contains("502")
        || lower.contains("504")
        || lower.contains("service unavailable")
        || lower.contains("bad gateway")
        || lower.contains("gateway timeout")
        || lower.contains("overloaded")
        || lower.contains("temporarily unavailable")
    {
        return true;
    }
    false
}

/// Result of a subtask execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTaskResult {
    /// Subtask ID
    pub subtask_id: String,

    /// Sub-agent ID that executed it
    pub subagent_id: String,

    /// Whether it succeeded
    pub success: bool,

    /// Result text
    pub result: String,

    /// Number of steps taken
    pub steps: usize,

    /// Tool calls made
    pub tool_calls: usize,

    /// Execution time (milliseconds)
    pub execution_time_ms: u64,

    /// Any error message
    pub error: Option<String>,

    /// Artifacts produced
    pub artifacts: Vec<String>,

    /// Number of retry attempts before this result was produced
    #[serde(default)]
    pub retry_count: u32,
}
