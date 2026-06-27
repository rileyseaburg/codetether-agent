//! In-place bearer-token helpers for [`BedrockAuth`].
//!
//! Kept separate from [`super::auth`] so the live mid-session refresh path
//! (swap a fresh SSO token into an active provider) has a focused home without
//! growing the credential-loading module.

use crate::provider::bedrock::auth::BedrockAuth;

impl BedrockAuth {
    /// Construct a bearer-token auth from a plain token string.
    pub fn bearer(token: String) -> Self {
        Self::BearerToken(std::sync::Arc::new(parking_lot::RwLock::new(token)))
    }

    /// Snapshot the current bearer token, if this is bearer auth.
    pub fn current_bearer(&self) -> Option<String> {
        match self {
            Self::BearerToken(cell) => Some(cell.read().clone()),
            Self::SigV4(_) => None,
        }
    }

    /// Atomically replace the bearer token in place. No-op for SigV4 auth.
    pub fn set_bearer(&self, new_token: String) {
        if let Self::BearerToken(cell) = self {
            *cell.write() = new_token;
        }
    }
}
