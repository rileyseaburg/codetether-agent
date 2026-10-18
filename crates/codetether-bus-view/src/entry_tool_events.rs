//! Remaining tool-output and thinking entry variants.
use super::{entry_parts::EntryParts, entry_tool_output, entry_tool_thinking};
use codetether_bus_core::BusMessage;
pub(super) fn entry_parts(message: &BusMessage) -> EntryParts {
    if let BusMessage::ToolOutputFull {
        agent_id,
        tool_name,
        output,
        success,
        step,
    } = message
    {
        return entry_tool_output::full(agent_id, tool_name, output, *success, *step);
    }
    if let BusMessage::AgentThinking {
        agent_id,
        thinking,
        step,
    } = message
    {
        return entry_tool_thinking::thinking(agent_id, thinking, *step);
    }
    unreachable!("tool entry builder received a non-tool bus message");
}
