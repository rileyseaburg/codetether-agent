//! Authenticated JSON request dispatch.
use crate::*;
impl VertexGlmProvider {
    pub(crate) async fn send(
        &self,
        url: &str,
        access_token: &str,
        body: &Value,
    ) -> reqwest::Result<reqwest::Response> {
        self.client
            .post(url)
            .bearer_auth(access_token)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
    }
}
