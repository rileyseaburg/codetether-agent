//! receive filter operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Receive the next envelope whose topic starts with the given prefix.
    pub async fn recv_topic(&mut self, prefix: &str) -> Option<BusEnvelope> {
        loop {
            match self.recv().await {
                Some(env) if env.topic.starts_with(prefix) => return Some(env),
                Some(_) => continue, // wrong topic, skip
                None => return None,
            }
        }
    }

    /// Receive the next envelope addressed to this agent.
    pub async fn recv_mine(&mut self) -> Option<BusEnvelope> {
        let prefix = format!("agent.{}", self.agent_id);
        self.recv_topic(&prefix).await
    }
}
