//! Service-account JWT authentication and per-instance OAuth token caching.
//!
//! ServiceAccountAuth owns credential parsing, signing, and cached tokens.
//! Provider adapters retain their endpoint construction and HTTP client policy.
use anyhow::{Context, Result};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
mod assertion;
mod construct;
mod exchange;
mod state;
mod token;
use state::*;
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const VERTEX_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";
/// Service-account credentials and a per-provider OAuth token cache.
///
/// # Examples
///
///     use codetether_google_auth::ServiceAccountAuth;
///     assert!(ServiceAccountAuth::from_json("{}", None).is_err());
pub struct ServiceAccountAuth {
    sa_key: ServiceAccountKey,
    encoding_key: EncodingKey,
    cached_token: Arc<RwLock<Option<CachedToken>>>,
}
impl ServiceAccountAuth {
    /// Return the account email for provider diagnostics without exposing keys.
    pub fn client_email(&self) -> &str {
        &self.sa_key.client_email
    }
}
