//! Exhaustive routing to focused per-message training projections.
use super::*;
include!("modules.rs");
macro_rules! dispatch_records {
    ($message:expr, $meta:expr; $( $variant:ident => $builder:ident ( $($field:ident),* ) ),* $(,)?) => {
        match $message {
            $(BusMessage::$variant { $($field,)* .. } => $builder::build($($field,)* $meta),)*
        }
    };
}
pub(super) fn envelope_to_training_record(env: &BusEnvelope) -> TrainingRecord {
    let meta = super::provenance::metadata(env);

    dispatch_records!(&env.message, meta;
    AgentReady => agent_ready(agent_id, capabilities),
    AgentToolCatalog => agent_tool_catalog(tools),
    AgentShutdown => agent_shutdown(agent_id),
    AgentMessage => agent_message(from, to, parts),
    TaskUpdate => task_update(task_id, state, message),
    ArtifactUpdate => artifact_update(task_id, artifact),
    SharedResult => shared_result(key, value, tags),
    ToolRequest => tool_request(request_id, tool_name, arguments),
    ToolResponse => tool_response(request_id, tool_name, result, success),
    ToolOutputFull => tool_output_full(agent_id, tool_name, output, success, step),
    Heartbeat => heartbeat(),
    RalphLearning => ralph_learning(prd_id, story_id, iteration, learnings, context),
    RalphHandoff => ralph_handoff(prd_id, from_story, to_story, context, progress_summary),
    RalphProgress => ralph_progress(prd_id, passed, total, iteration, status),
    AgentThinking => agent_thinking(agent_id, thinking, step),
    VoiceSessionStarted => voice_session_started(room_name, agent_id, voice_id),
    VoiceTranscript => voice_transcript(room_name, text, role, is_final),
    VoiceAgentStateChanged => voice_agent_state_changed(room_name, state),
    VoiceSessionEnded => voice_session_ended(room_name, reason),
    AgentSpeech => agent_speech(act, from, to, conversation_id, content),
    UserPrompt => user_prompt(agent_id, text, session_id),
    )
}
