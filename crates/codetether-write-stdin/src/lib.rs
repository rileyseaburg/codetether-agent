//! Incremental input and polling for persistent command sessions.

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use codetether_command_session::Registry;
use codetether_tool_core::{Tool, ToolResult};

#[path = "execute.rs"]
mod execute;
#[path = "input.rs"]
mod input;
#[path = "parameters.rs"]
mod parameters;

/// Writes to or polls an existing persistent command session.
///
/// # Examples
/// ```rust
/// use std::sync::Arc;
/// use codetether_command_session::Registry;
/// use codetether_tool_core::Tool;
/// let tool = codetether_write_stdin::WriteStdinTool::new(Arc::new(Registry::default()));
/// assert_eq!(tool.id(), "write_stdin");
/// ```
pub struct WriteStdinTool {
    sessions: Arc<Registry>,
}

impl WriteStdinTool {
    /// Bind the tool to the registry used when commands are spawned.
    ///
    /// # Arguments
    /// * `sessions` - Shared persistent-command registry.
    /// # Returns
    /// A tool that writes and polls commands retained in this registry.
    /// # Examples
    /// ```rust
    /// let sessions = std::sync::Arc::new(codetether_command_session::Registry::default());
    /// let _tool = codetether_write_stdin::WriteStdinTool::new(sessions);
    /// ```
    pub fn new(sessions: Arc<Registry>) -> Self {
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
