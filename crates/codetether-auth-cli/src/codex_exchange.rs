//! Optional API-key exchange and expected OAuth fallback classification.
use crate::is_expected_codex_id_token_exchange_fallback;
use codetether_provider_codex::{OAuthCredentials, OpenAiCodexProvider};

pub(super) async fn key(credentials: &OAuthCredentials) -> (Option<String>, bool) {
    let mut expected_token_exchange_fallback = false;
    let api_key = if let Some(id_token) = credentials.id_token.as_deref() {
        match OpenAiCodexProvider::exchange_id_token_for_api_key(id_token).await {
            Ok(key) => Some(key),
            Err(error) => {
                if is_expected_codex_id_token_exchange_fallback(&error) {
                    expected_token_exchange_fallback = true;
                    tracing::info!(
                        error = %error,
                        "Expected id_token exchange fallback; using OAuth access token for Codex backend"
                    );
                } else {
                    tracing::warn!(
                        error = %error,
                        "Failed to exchange id_token for OpenAI API key; falling back to OAuth access token"
                    );
                }
                None
            }
        }
    } else {
        tracing::warn!(
            "OAuth token exchange did not return an id_token; cannot derive OpenAI API key"
        );
        None
    };

    (api_key, expected_token_exchange_fallback)
}
