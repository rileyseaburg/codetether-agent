//! Incremental input and polling for persistent command sessions.

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::tool::command_session::Registry;
use crate::tool::{Tool, ToolResult};

#[path = "write_stdin/execute.rs"]
mod execute;
#[path = "write_stdin/input.rs"]
mod input;
#[path = "write_stdin/parameters.rs"]
mod parameters;

/// Writes to or polls a process created by [`ExecCommandTool`](crate::tool::exec_command::ExecCommandTool).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::ToolRegistry;
///
/// let registry = ToolRegistry::with_defaults();
/// let tool = registry.get("write_stdin").expect("write stdin is registered");
/// assert_eq!(tool.id(), "write_stdin");
/// ```
pub struct WriteStdinTool {
    sessions: Arc<Registry>,
}

impl WriteStdinTool {
    pub(crate) fn new(sessions: Arc<Registry>) -> Self {
        Self { sessions }
    }
}

#[async_trait]
impl Tool for WriteStdinTool {
    fn id(&self) -> &str {
        "write_stdin"
    }
    fn name(&self) -> &str {
        "Write Stdin"
    }
    fn description(&self) -> &str {
        "Writes characters to an existing command session and returns recent output."
    }
    fn parameters(&self) -> serde_json::Value {
        parameters::schema()
    }
    async fn execute(&self, args: serde_json::Value) -> Result<ToolResult> {
        execute::run(self, args).await
    }
}
