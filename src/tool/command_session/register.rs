//! Native command tool constructors for shared process-registry assembly.
use super::Registry;
use crate::tool::{
    Tool, ToolRegistry, bash::BashTool, exec_command::ExecCommandTool, write_stdin::WriteStdinTool,
};
use std::{path::PathBuf, sync::Arc};
struct Native<'a>(&'a mut ToolRegistry);
impl codetether_command_session::tool_registration::ToolHost for Native<'_> {
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
pub(crate) fn register(registry: &mut ToolRegistry, cwd: Option<PathBuf>) {
    codetether_command_session::tool_registration::register(&mut Native(registry), cwd);
}
