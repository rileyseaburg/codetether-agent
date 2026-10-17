//! Copilot request headers and dispatch.
use crate::*;
impl CopilotProvider {
    pub(super) async fn send(
        &self,
        body: &Value,
        is_agent: bool,
        has_vision: bool,
    ) -> Result<reqwest::Response> {
        let mut req = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .header("Openai-Intent", "conversation-edits")
            .header("User-Agent", &self.user_agent)
            .header("X-Initiator", if is_agent { "agent" } else { "user" });

        if has_vision {
            req = req.header("Copilot-Vision-Request", "true");
        }

        req.json(body)
            .send()
            .await
            .context("Failed to send Copilot request")
    }
}
