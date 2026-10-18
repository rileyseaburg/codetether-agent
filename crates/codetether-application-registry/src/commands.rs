//! Construct command tools sharing one process registry and register them in order.
//!
//! [`register`] installs bash, exec and stdin using the same process-session
//! registry for exec and stdin, with the optional working directory preserved.
use crate::{Application, ToolRegistry};
use codetether_bash_tool::BashTool;
pub use codetether_command_session::spawn as command;
use codetether_command_session::tool_registration::ToolHost;
pub use codetether_command_session::{Poll, Registry, Running, SpawnMetadata, tool_result};
use codetether_exec_command::ExecCommandTool;
use codetether_tool_core::Tool;
use codetether_write_stdin::WriteStdinTool;
use std::{path::PathBuf, sync::Arc};
struct Commands<'a, H: Application>(&'a mut ToolRegistry<H>);
impl<H: Application> ToolHost for Commands<'_, H> {
    fn bash(cwd: Option<PathBuf>) -> Arc<dyn Tool> {
        match cwd {
            Some(path) => Arc::new(BashTool::with_cwd(path)),
            None => Arc::new(BashTool::new()),
        }
    }
    fn exec(sessions: Arc<Registry>, cwd: Option<PathBuf>) -> Arc<dyn Tool> {
        Arc::new(ExecCommandTool::new(sessions, cwd))
    }
    fn stdin(sessions: Arc<Registry>) -> Arc<dyn Tool> {
        Arc::new(WriteStdinTool::new(sessions))
    }
    fn register(&mut self, tool: Arc<dyn Tool>) {
        self.0.register(tool);
    }
}
/// Install bash, exec and stdin in order, sharing their process-session state.
/// # Arguments
/// `registry` receives the tools; `cwd` scopes bash and exec when supplied.
/// # Returns
/// Completes after the three tools have been registered.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_registry::Application>(registry: &mut codetether_application_registry::ToolRegistry<H>) {
/// codetether_application_registry::commands::register(registry, None);
/// # }
/// ```
pub fn register<H: Application>(registry: &mut ToolRegistry<H>, cwd: Option<PathBuf>) {
    codetether_command_session::tool_registration::register(&mut Commands(registry), cwd);
}

#[cfg(test)]
mod native_tests;
