//! [`BusHandle`] speech send/receive helpers for the agent language.

use super::super::{BusHandle, BusMessage};
use super::SpeechAct;

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
