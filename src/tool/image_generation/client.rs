use super::{
    auth::ImagesAuth,
    response::{ApiErrorEnvelope, ImagesResponse},
};
use anyhow::{Context, Result, bail};
use reqwest::Client;

pub(super) struct ImagesClient {
    http: Client,
}

impl ImagesClient {
    pub(super) fn new() -> Self {
        Self {
            http: Client::new(),
        }
    }

    pub(super) async fn request(
        &self,
        auth: &ImagesAuth,
        prompt: &str,
        images: Vec<String>,
    ) -> Result<String> {
        let (endpoint, body) = super::request_body::build(prompt, images);
        let request = self.http.post(auth.endpoint(endpoint));
        let response = auth
            .authorize(request)
            .json(&body)
            .send()
            .await
            .context("failed to call OpenAI Images API")?;
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .context("failed to read Images API response")?;
        if !status.is_success() {
            let message = serde_json::from_slice::<ApiErrorEnvelope>(&bytes)
                .map(ApiErrorEnvelope::message)
                .unwrap_or_else(|_| status.to_string());
            bail!("OpenAI Images API request failed: {message}");
        }
        serde_json::from_slice::<ImagesResponse>(&bytes)
            .context("invalid OpenAI Images API response")?
            .into_base64()
    }
}
