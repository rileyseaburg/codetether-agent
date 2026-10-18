//! Browser OAuth flow, state verification and token exchange.
use crate::{
    CODEX_CALLBACK_TIMEOUT_SECS, CODEX_CALLBACK_TIMEOUT_SSH_SECS, authenticate_codex_device_code,
    store_codex_credentials,
};
use anyhow::{Context, Result};
use codetether_provider_codex::OpenAiCodexProvider;
use tokio::time::Duration;

pub(super) async fn run(application_version: &str) -> Result<()> {
    let (authorization_url, code_verifier, expected_state) =
        OpenAiCodexProvider::get_authorization_url();

    let is_ssh_session = crate::codex_instructions::show(&authorization_url);

    let callback_timeout = if is_ssh_session {
        Duration::from_secs(CODEX_CALLBACK_TIMEOUT_SSH_SECS)
    } else {
        Duration::from_secs(CODEX_CALLBACK_TIMEOUT_SECS)
    };
    let Some((authorization_code, returned_state)) =
        crate::codex_callback::read(callback_timeout, is_ssh_session).await?
    else {
        let credentials = authenticate_codex_device_code(application_version).await?;
        return store_codex_credentials(credentials).await;
    };

    if returned_state != expected_state {
        anyhow::bail!(
            "OAuth state mismatch. Retry `codetether auth codex` and paste the callback URL from the same login attempt."
        );
    }

    let credentials = OpenAiCodexProvider::exchange_code(&authorization_code, &code_verifier)
        .await
        .context("Failed to exchange ChatGPT OAuth code for Codex tokens")?;

    store_codex_credentials(credentials).await
}
