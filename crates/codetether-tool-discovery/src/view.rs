//! Discovery response record with derived execution availability.
use crate::{RegisteredTool, contract as tool_contract};
use serde::Serialize;

#[derive(Serialize)]
/// Flattened registration with its derived agent-executable flag.
/// Construct with RegisteredToolView::from(tool) before serializing a response.
pub struct RegisteredToolView {
    #[serde(flatten)]
    tool: RegisteredTool,
    agent_executable: bool,
}

impl From<RegisteredTool> for RegisteredToolView {
    fn from(tool: RegisteredTool) -> Self {
        let agent_executable = tool_contract::is_agent_executable(&tool.execution_mode);
        Self {
            tool,
            agent_executable,
        }
    }
}
