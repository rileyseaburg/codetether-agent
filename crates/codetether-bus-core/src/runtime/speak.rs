//! speak operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Send a typed [`SpeechAct`] to `to` (an agent id or `"all"`) within a
    /// conversation thread.  Returns the number of receivers reached.
    pub fn speak(
        &self,
        act: SpeechAct,
        to: &str,
        conversation_id: &str,
        content: impl Into<String>,
    ) -> usize {
        let topic = if to == "all" {
            "broadcast".to_string()
        } else {
            format!("agent.{to}")
        };
        self.send(
            topic,
            BusMessage::AgentSpeech {
                act: act.as_str().to_string(),
                from: self.agent_id().to_string(),
                to: to.to_string(),
                conversation_id: conversation_id.to_string(),
                content: content.into(),
            },
        )
    }
}
