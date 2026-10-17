//! Authenticated HTTP JSON dispatch.
use crate::*;
impl Glm5Provider {
    pub(crate) async fn send(&self, body: &Value) -> reqwest::Result<reqwest::Response> {
        self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
    }
}
