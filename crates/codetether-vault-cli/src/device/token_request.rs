//! One credential-private device polling request.
use super::token_reply::{Outcome, Reply};
use anyhow::Result;
pub(super) async fn request(
    client: &reqwest::Client,
    metadata: &super::discovery::Metadata,
    grant: &super::start::Grant,
    client_id: &str,
) -> Result<Outcome> {
    let response = client
        .post(&metadata.token_endpoint)
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("client_id", client_id),
            ("device_code", grant.device_code.as_str()),
        ])
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Device token endpoint is unavailable"))?;
    let status = response.status();
    let reply: Reply = response
        .json()
        .await
        .map_err(|_| anyhow::anyhow!("Invalid device response"))?;
    super::token_reply::interpret(reply, status)
}
