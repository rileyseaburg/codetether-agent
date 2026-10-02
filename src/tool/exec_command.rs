//! Persistent command execution compatible with Codex-style tool calling.

use std::path::PathBuf;
use std::sync::Arc;

use crate::tool::command_session::Registry;

#[path = "exec_command/diagnostics.rs"]
mod diagnostics;
#[path = "exec_command/environment.rs"]
mod environment;
#[path = "exec_command/execute.rs"]
mod execute;
#[path = "exec_command/input.rs"]
mod input;
#[path = "exec_command/parameters.rs"]
mod parameters;
#[path = "exec_command/policy.rs"]
mod policy;
#[path = "exec_command/shell.rs"]
mod shell;
#[path = "exec_command/tool.rs"]
mod tool;

/// Starts commands and yields a session identifier when they remain active.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::ToolRegistry;
///
/// let registry = ToolRegistry::with_defaults();
/// let tool = registry.get("exec_command").expect("exec command is registered");
/// assert_eq!(tool.id(), "exec_command");
/// ```
pub struct ExecCommandTool {
    sessions: Arc<Registry>,
    default_cwd: Option<PathBuf>,
}

impl ExecCommandTool {
    pub(crate) fn new(sessions: Arc<Registry>, default_cwd: Option<PathBuf>) -> Self {
        Self {
            sessions,
            default_cwd,
        }
    }
}
