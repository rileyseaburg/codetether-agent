//! Compatibility adapter for native tool-result recording tests.
use crate::{agent::Agent, provider::ContentPart, session::Session, tool::ToolResult};
pub(super) fn record_results(
    session: &mut Session,
    tool_calls: Vec<ContentPart>,
    results: Vec<ToolResult>,
) {
    codetether_agent_replay::record_results::<Agent>(session, tool_calls, results);
}
