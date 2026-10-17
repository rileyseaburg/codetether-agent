//! Codex OAuth credential exchange and authorization helpers.
//!
//! [CodexOAuth] owns wire exchanges and account-claim parsing; callers retain
//! credential caches, cross-process locking, and persistence.
use anyhow::{Context, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
mod credentials;
mod pkce;
mod wire;
pub use credentials::OAuthCredentials;
pub use pkce::PkcePair;
use wire::{OAuthTokenResponse, RefreshTokenResponse};

const AUTH_ISSUER: &str = "https://auth.openai.com";

const AUTHORIZE_URL: &str = "https://auth.openai.com/oauth/authorize";

const TOKEN_URL: &str = "https://auth.openai.com/oauth/token";

// Public clients cannot treat this OAuth client ID as a secret. Backend use is
// still gated by `CODETETHER_OPENAI_CODEX_ALLOW_CHATGPT_BACKEND`.
const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";

const REDIRECT_URI: &str = "http://localhost:1455/auth/callback";

const SCOPE: &str = "openid profile email offline_access";
/// Namespace for OAuth authorization, exchange, and claim helpers.
///
/// # Examples
/// ```
/// assert!(!codetether_codex_auth::CodexOAuth::oauth_client_id().is_empty());
/// ```
pub struct CodexOAuth;
mod exchange_code;
mod exchange_code_with_redirect_uri;
mod exchange_id_token_for_api_key;
mod extract_chatgpt_account_id;
mod extract_chatgpt_account_id_from_jwt;
mod generate_pkce;
mod generate_state;
mod get_authorization_url;
mod oauth_client_id;
mod oauth_expiry;
mod oauth_issuer;
mod refresh_access_token;

/// Shared credential refresh coordination and persistence.
pub mod lifecycle;
