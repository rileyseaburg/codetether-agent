use codetether_tool_core::ToolResult;

/// Message types for swarm coordination
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// use codetether_swarm_core::SwarmMessage;
/// let message = SwarmMessage::TaskCompleted { task_id: "task-1".into(), result: "done".into() };
/// if let SwarmMessage::TaskCompleted { result, .. } = message { assert_eq!(result, "done"); }
/// ```
pub enum SwarmMessage {
    /// Execute a task
    ExecuteTask {
        task_id: String,
        instruction: String,
    },
    /// Report progress
    Progress {
        task_id: String,
        progress: f32,
        message: String,
    },
    /// Task completed
    TaskCompleted { task_id: String, result: String },
    /// Task failed
    TaskFailed { task_id: String, error: String },
    /// Request tool execution
    ToolRequest {
        tool_id: String,
        arguments: serde_json::Value,
    },
    /// Tool response
    ToolResponse { tool_id: String, result: ToolResult },
}
