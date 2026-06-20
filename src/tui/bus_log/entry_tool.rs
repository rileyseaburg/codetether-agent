//! Router for tool request and output entries.

use crate::bus::BusMessage;

use super::{entry_parts::EntryParts, entry_tool_output, entry_tool_request, entry_tool_thinking};

pub(super) fn entry_parts(message: &BusMessage) -> EntryParts {
    if let BusMessage::ToolRequest {
        request_id,
        agent_id,
        tool_name,
        arguments,
        step,
    } = message
    {
        return entry_tool_request::request(request_id, agent_id, tool_name, arguments, *step);
    }
    if let BusMessage::ToolResponse {
        request_id,
        agent_id,
        tool_name,
        result,
        success,
        step,
    } = message
    {
        return entry_tool_output::response(
            request_id, agent_id, tool_name, result, *success, *step,
        );
    }
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
