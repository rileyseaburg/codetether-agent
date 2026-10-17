//! Loop-step attribution for training provenance.
use super::*;

/// Convert a `BusEnvelope` into a `TrainingRecord`.
pub(super) fn envelope_step(message: &BusMessage) -> Option<usize> {
    match message {
        BusMessage::ToolRequest { step, .. }
        | BusMessage::ToolResponse { step, .. }
        | BusMessage::ToolOutputFull { step, .. }
        | BusMessage::AgentThinking { step, .. }
        | BusMessage::RalphLearning {
            iteration: step, ..
        }
        | BusMessage::RalphProgress {
            iteration: step, ..
        } => Some(*step),
        BusMessage::AgentReady { .. }
        | BusMessage::AgentToolCatalog { .. }
        | BusMessage::AgentShutdown { .. }
        | BusMessage::AgentMessage { .. }
        | BusMessage::TaskUpdate { .. }
        | BusMessage::ArtifactUpdate { .. }
        | BusMessage::SharedResult { .. }
        | BusMessage::Heartbeat { .. }
        | BusMessage::RalphHandoff { .. }
        | BusMessage::VoiceSessionStarted { .. }
        | BusMessage::VoiceTranscript { .. }
        | BusMessage::VoiceAgentStateChanged { .. }
        | BusMessage::VoiceSessionEnded { .. }
        | BusMessage::AgentSpeech { .. }
        | BusMessage::UserPrompt { .. } => None,
    }
}
