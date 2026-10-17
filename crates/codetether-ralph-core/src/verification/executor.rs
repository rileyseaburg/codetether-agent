//! Host boundary for policy-controlled command execution.
use anyhow::Result;
use std::path::Path;
/// Output returned by the host shell tool.
///
/// # Examples
/// ```
/// let output = codetether_ralph_core::verification::ShellOutput { success: true, output: "ok".into() };
/// assert!(output.success);
/// ```
pub struct ShellOutput {
    /// Whether the executed command succeeded.
    pub success: bool,
    /// Combined output text in the host tool's original format.
    pub output: String,
}
/// Host shell executor, including the host's policy and approval checks.
///
/// # Implementors
/// Implementations must apply their command policy before executing a command.
#[async_trait::async_trait]
pub trait ShellExecutor: Sync {
    /// Run one shell command in the supplied working directory.
    ///
    /// # Arguments
    /// * `cwd` - Resolved working directory.
    /// * `command` - Command text from the story verification step.
    /// # Returns
    /// Tool success and output, before verification assertions are applied.
    /// # Errors
    /// Returns host policy denials or tool execution errors.
    async fn execute(&self, cwd: &Path, command: &str) -> Result<ShellOutput>;
}
