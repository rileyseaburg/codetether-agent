//! Assembly of command tools sharing one process-session registry.
use crate::Registry;
use codetether_tool_core::Tool;
use std::{path::PathBuf, sync::Arc};
/// Native constructors for the command tool trio.
pub trait ToolHost {
    /// Construct bash with the optional working directory.
    fn bash(cwd: Option<PathBuf>) -> Arc<dyn Tool>;
    /// Construct exec with the shared process registry and optional directory.
    fn exec(sessions: Arc<Registry>, cwd: Option<PathBuf>) -> Arc<dyn Tool>;
    /// Construct stdin with the same process registry as exec.
    fn stdin(sessions: Arc<Registry>) -> Arc<dyn Tool>;
    /// Add the constructed tool to the application's tool registry.
    fn register(&mut self, tool: Arc<dyn Tool>);
}
/// Construct bash, exec and stdin in order, sharing one process registry.
/// # Arguments
/// * `host` — Native constructors and destination registry.
/// * `cwd` — Optional working directory used by bash and exec.
/// # Returns
/// Registers three tools with the original ownership and clone behavior.
/// # Examples
/// ```no_run
/// # fn register(host: &mut impl codetether_command_session::tool_registration::ToolHost) {
/// codetether_command_session::tool_registration::register(host, None);
/// # }
/// ```
pub fn register<H: ToolHost>(host: &mut H, cwd: Option<PathBuf>) {
    let sessions = Arc::new(Registry::default());
    let bash = H::bash(cwd.clone());
    host.register(bash);
    host.register(H::exec(Arc::clone(&sessions), cwd));
    host.register(H::stdin(sessions));
}
