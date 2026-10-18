//! Persistent command execution compatible with Codex-style tool calling.

use std::path::PathBuf;
use std::sync::Arc;

use codetether_command_session::Registry;

#[path = "diagnostics.rs"]
mod diagnostics;
use codetether_command_environment as environment;
#[path = "execute.rs"]
mod execute;
#[path = "input.rs"]
mod input;
#[path = "parameters.rs"]
mod parameters;
#[path = "policy.rs"]
pub mod policy;
#[path = "shell.rs"]
mod shell;
#[path = "tool.rs"]
mod tool;

/// Starts commands and returns a session ID when they remain active.
///
/// # Examples
/// ```rust
/// let sessions = std::sync::Arc::new(codetether_command_session::Registry::default());
/// let _tool = codetether_exec_command::ExecCommandTool::new(sessions, None);
/// ```
pub struct ExecCommandTool {
    sessions: Arc<Registry>,
    default_cwd: Option<PathBuf>,
}

impl ExecCommandTool {
    /// Bind command execution to its shared registry and default working directory.
    ///
    /// # Arguments
    /// * `sessions` - Registry also used by the stdin polling tool.
    /// * `default_cwd` - Workspace directory used when callers omit a workdir.
    /// # Returns
    /// The configured command tool.
    pub fn new(sessions: Arc<Registry>, default_cwd: Option<PathBuf>) -> Self {
        Self {
            sessions,
            default_cwd,
        }
    }
}

#[cfg(test)]
mod native_tests;
