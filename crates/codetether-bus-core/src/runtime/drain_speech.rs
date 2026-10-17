//! drain speech operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Drain all queued speech acts addressed to this agent (or broadcast),
    /// non-blocking.  Returns `(act, from, conversation_id, content)` tuples.
    pub fn drain_speech(&mut self) -> Vec<(SpeechAct, String, String, String)> {
        let me = self.agent_id().to_string();
        let mut out = Vec::new();
        while let Some(env) = self.try_recv() {
            if let BusMessage::AgentSpeech {
                act,
                from,
                to,
                conversation_id,
                content,
            } = env.message
                && (to == me || to == "all")
            {
                out.push((
                    SpeechAct::from_str_lenient(&act),
                    from,
                    conversation_id,
                    content,
                ));
            }
        }
        out
    }
}
