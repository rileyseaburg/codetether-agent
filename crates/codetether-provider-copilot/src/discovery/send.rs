//! Discovery request with caller identity.
use crate::*;
impl CopilotProvider {
    pub(super) async fn discovery_send(&self) -> Option<reqwest::Response> {
        match self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", &self.user_agent)
            .send()
            .await
        {
            Ok(r) => Some(r),
            Err(e) => {
                tracing::warn!(provider = %self.provider_name, error = %e, "Failed to fetch Copilot models endpoint");
                return None;
            }
        }
    }
}
