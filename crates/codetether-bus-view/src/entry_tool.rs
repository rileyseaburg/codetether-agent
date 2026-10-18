//! Router for tool request and output entries.

use codetether_bus_core::BusMessage;

use super::{entry_parts::EntryParts, entry_tool_output, entry_tool_request};

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
    super::entry_tool_events::entry_parts(message)
}
