//! Typed registration and token responses; deliberately no Debug derives.
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Registration {
    pub(super) client_id: String,
    pub(super) token_endpoint_auth_method: String,
    pub(super) redirect_uris: Vec<String>,
}

#[derive(Deserialize)]
pub(super) struct TokenResponse {
    pub(super) access_token: String,
    pub(super) token_type: String,
    pub(super) expires_in: u64,
    pub(super) refresh_token: Option<String>,
    pub(super) scope: Option<String>,
}
