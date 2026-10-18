//! Execute a specialist turn and retain its session on either outcome.
use crate::{Host, Session, state::Runtime};
impl<H: Host> Runtime<H> {
    pub(super) async fn prompt(&mut self, to: &str, prompt_input: &str) -> Option<String> {
        let Some(mut session) = self.team.sessions.remove(to) else {
            self.status = "agent_error".to_string();
            self.failure_note = Some(format!("Relay agent @{to} session was unavailable."));
            return None;
        };

        let output = match session.prompt(&prompt_input).await {
            Ok(response) => response.text,
            Err(err) => {
                self.status = "agent_error".to_string();
                self.failure_note = Some(format!("Relay agent @{to} failed: {err}"));
                self.team.sessions.insert(to.to_string(), session);
                return None;
            }
        };

        self.team.sessions.insert(to.to_string(), session);

        Some(output)
    }
}
