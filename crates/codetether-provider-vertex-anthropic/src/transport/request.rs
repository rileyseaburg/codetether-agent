//! One buffered HTTP send.
use crate::*;
impl VertexAnthropicProvider {
    pub(super) async fn send_once(
        &self,
        url: &str,
        access_token: &str,
        body: &Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .post(url)
            .bearer_auth(&access_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
    }
}
