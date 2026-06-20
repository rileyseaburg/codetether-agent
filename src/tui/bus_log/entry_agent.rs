//! Router for agent lifecycle, message, and heartbeat entries.

use crate::bus::BusMessage;

use super::{entry_agent_lifecycle, entry_agent_message, entry_parts::EntryParts, entry_speech};

pub(super) fn entry_parts(message: &BusMessage) -> EntryParts {
    if let BusMessage::AgentReady {
        agent_id,
        capabilities,
    } = message
    {
        return entry_agent_lifecycle::ready(agent_id, capabilities);
    }
    if let BusMessage::AgentShutdown { agent_id } = message {
        return entry_agent_lifecycle::shutdown(agent_id);
    }
    if let BusMessage::AgentMessage { from, to, parts } = message {
        return entry_agent_message::message(from, to, parts);
    }
    if let BusMessage::Heartbeat { agent_id, status } = message {
        return entry_agent_lifecycle::heartbeat(agent_id, status);
    }
    if let BusMessage::AgentSpeech {
        act,
        from,
        to,
        conversation_id,
        content,
    } = message
    {
        return entry_speech::speech(act, from, to, conversation_id, content);
    }
    unreachable!("agent entry builder received a non-agent bus message");
}
