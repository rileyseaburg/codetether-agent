//! Exchange the JWT assertion and decode the token response.
use super::*;
#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub(crate) access_token: String,
    #[serde(default)]
    pub(crate) expires_in: Option<u64>,
}
impl ServiceAccountAuth {
    pub(super) async fn exchange(
        &self,
        client: &Client,
        token_uri: &str,
        assertion: &str,
    ) -> Result<TokenResponse> {
        // Exchange JWT for access token
        let form_body = format!(
            "grant_type={}&assertion={}",
            urlencoding::encode("urn:ietf:params:oauth:grant-type:jwt-bearer"),
            urlencoding::encode(&assertion),
        );
        let response = client
            .post(token_uri)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .context("Failed to exchange JWT for access token")?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("Failed to read token response")?;

        if !status.is_success() {
            anyhow::bail!("GCP token exchange failed: {status} {body}");
        }

        let token_resp: TokenResponse =
            serde_json::from_str(&body).context("Failed to parse GCP token response")?;

        Ok(token_resp)
    }
}
