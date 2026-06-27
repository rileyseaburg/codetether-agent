//! Mid-session bearer-token refresh on Bedrock stream auth failures.
//!
//! When a live converse-stream request is rejected with 401/403 and the
//! provider uses bearer auth, this attempts a browser-free SSO refresh, swaps
//! the new token into the provider's auth cell, and signals a retry — so an
//! active TUI session recovers without restarting.

use crate::provider::bedrock::{BedrockProvider, sso_refresh};
use reqwest::StatusCode;

/// Whether the status indicates an authentication/authorization failure.
pub(super) fn is_auth_failure(status: StatusCode) -> bool {
    matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN)
}

/// Try to refresh the provider's bearer token in place.
///
/// Returns `true` when a fresh token was minted and swapped in (caller should
/// retry the request), `false` when refresh was impossible (SigV4 auth, no SSO
/// metadata, or expired refresh token).
pub(super) async fn try_refresh(provider: &BedrockProvider) -> bool {
    if provider.auth.current_bearer().is_none() {
        return false;
    }
    match sso_refresh::refresh_now(true).await {
        Ok(refreshed) => {
            provider.auth.set_bearer(refreshed.token);
            tracing::info!(provider = "bedrock", "refreshed bearer token mid-session");
            true
        }
        Err(e) => {
            tracing::warn!(provider = "bedrock", error = %e, "mid-session refresh failed");
            false
        }
    }
}
