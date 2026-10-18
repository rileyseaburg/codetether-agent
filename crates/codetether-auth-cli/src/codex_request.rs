//! Authentication codex_request responsibilities.
use anyhow::Result;
use codetether_provider_codex::OpenAiCodexProvider;
use reqwest::Client;

pub(super) async fn request_codex_device_code(
    client: &Client,
    issuer: &str,
    user_agent: &str,
) -> Result<codetether_device_auth::CodexDeviceCodeResponse> {
    codetether_device_auth::request_codex_device_code(
        client,
        issuer,
        user_agent,
        OpenAiCodexProvider::oauth_client_id(),
    )
    .await
}
