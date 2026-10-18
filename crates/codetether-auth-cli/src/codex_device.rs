//! Authentication codex_device responsibilities.
use crate::{poll_for_codex_authorization_code, request_codex_device_code};
use anyhow::{Context, Result};
use codetether_provider_codex::{OAuthCredentials, OpenAiCodexProvider};
use reqwest::Client;

pub(super) async fn authenticate_codex_device_code(
    application_version: &str,
) -> Result<OAuthCredentials> {
    let client = Client::new();
    let issuer = OpenAiCodexProvider::oauth_issuer().trim_end_matches('/');
    let user_agent = format!("codetether-agent/{}", application_version);
    let device_code = request_codex_device_code(&client, issuer, &user_agent).await?;

    println!("OpenAI Codex device authentication");
    println!("Open this URL: {issuer}/codex/device");
    println!("Enter code: {}", device_code.user_code);
    println!("Waiting for authorization...");

    let code = poll_for_codex_authorization_code(&client, issuer, &user_agent, &device_code)
        .await
        .context("Timed out waiting for device authorization")?;

    let redirect_uri = format!("{issuer}/deviceauth/callback");
    OpenAiCodexProvider::exchange_code_with_redirect_uri(
        &code.authorization_code,
        &code.code_verifier,
        &redirect_uri,
    )
    .await
    .context("Failed to exchange device authorization code for Codex tokens")
}
