//! Exhaustive routing from bus message variants to entry builders.

use crate::bus::BusMessage;

use super::{
    entry_agent, entry_parts::EntryParts, entry_ralph, entry_task, entry_tool, entry_voice,
};

pub(super) fn entry_parts(message: &BusMessage) -> EntryParts {
    match message {
        BusMessage::AgentReady { .. }
        | BusMessage::AgentShutdown { .. }
        | BusMessage::AgentMessage { .. }
        | BusMessage::Heartbeat { .. }
        | BusMessage::AgentSpeech { .. } => entry_agent::entry_parts(message),
        BusMessage::TaskUpdate { .. }
        | BusMessage::ArtifactUpdate { .. }
        | BusMessage::SharedResult { .. } => entry_task::entry_parts(message),
        BusMessage::ToolRequest { .. }
        | BusMessage::ToolResponse { .. }
        | BusMessage::ToolOutputFull { .. }
        | BusMessage::AgentThinking { .. } => entry_tool::entry_parts(message),
        BusMessage::RalphLearning { .. }
        | BusMessage::RalphHandoff { .. }
        | BusMessage::RalphProgress { .. } => entry_ralph::entry_parts(message),
        BusMessage::VoiceSessionStarted { .. }
        | BusMessage::VoiceTranscript { .. }
        | BusMessage::VoiceAgentStateChanged { .. }
        | BusMessage::VoiceSessionEnded { .. } => entry_voice::entry_parts(message),
        BusMessage::UserPrompt { .. } => entry_agent::entry_parts(message),
    }
}
