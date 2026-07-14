//! Session-authoritative tool-input preparation for agents.

use crate::agent::Agent;
use crate::session::Session;
use crate::tool::ToolResult;

impl Agent {
    pub(super) async fn execute_tool_for_session(
        &self,
        session: &Session,
        name: &str,
        arguments: &str,
    ) -> ToolResult {
        let input = match prepare(session, arguments) {
            Ok(value) => value,
            Err(error) => return ToolResult::error(error),
        };
        self.execute_tool_value(name, input).await
    }
}

pub(super) fn prepare(session: &Session, arguments: &str) -> Result<serde_json::Value, String> {
    let input = serde_json::from_str(arguments)
        .map_err(|error| format!("Failed to parse arguments: {error}"))?;
    let cwd = session
        .metadata
        .directory
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new("."));
    Ok(crate::session::helper::runtime::enrich_tool_input_for_session(&input, cwd, session))
}
