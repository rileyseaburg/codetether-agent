//! Registration of command tools that share one process registry.

use std::path::PathBuf;
use std::sync::Arc;

use super::Registry;
use crate::tool::write_stdin::WriteStdinTool;
use crate::tool::{ToolRegistry, bash::BashTool, exec_command::ExecCommandTool};

pub(crate) fn register(registry: &mut ToolRegistry, cwd: Option<PathBuf>) {
    let sessions = Arc::new(Registry::default());
    let bash: Arc<dyn crate::tool::Tool> = match cwd.clone() {
        Some(path) => Arc::new(BashTool::with_cwd(path)),
        None => Arc::new(BashTool::new()),
    };
    registry.register(bash);
    registry.register(Arc::new(ExecCommandTool::new(Arc::clone(&sessions), cwd)));
    registry.register(Arc::new(WriteStdinTool::new(sessions)));
}
