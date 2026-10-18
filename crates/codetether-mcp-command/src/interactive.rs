//! Frontend execution and presentation of MCP slash-command results.
use crate::CommandOutput;
/// Native registry and presentation operations for an MCP command.
/// # Implementors
/// Frontends use their existing connection registry and system-message path.
#[allow(async_fn_in_trait)]
pub trait Host {
    /// Execute a command against the frontend's connection registry.
    async fn execute(&mut self, raw: &str) -> CommandOutput;
    /// Replace status text with the command outcome.
    fn set_status(&mut self, status: String);
    /// Append and reveal a system notice when the command supplies one.
    fn push_system_message(&mut self, message: String);
}
/// Execute a command, apply its status, then display its optional system notice.
/// # Arguments
/// * `host` — Existing registry and frontend effects.
/// * `raw` — MCP subcommand text.
/// # Returns
/// Updates the frontend using the completed command output.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_mcp_command::interactive::Host) {
/// codetether_mcp_command::interactive::handle(host, "servers").await;
/// # }
/// ```
pub async fn handle(host: &mut impl Host, raw: &str) {
    let output = host.execute(raw).await;
    host.set_status(output.status);
    if let Some(message) = output.message {
        host.push_system_message(message);
    }
}
