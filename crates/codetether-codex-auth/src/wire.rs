//! Private OAuth token response representations.
use serde::Deserialize;
#[derive(Deserialize)]
pub(crate) struct OAuthTokenResponse {
    #[serde(default)]
    pub(crate) id_token: Option<String>,
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
    pub(crate) expires_in: u64,
}

#[derive(Deserialize)]
pub(crate) struct RefreshTokenResponse {
    pub(crate) access_token: String,
    #[serde(default)]
    pub(crate) refresh_token: Option<String>,
    #[serde(default)]
    pub(crate) id_token: Option<String>,
    pub(crate) expires_in: u64,
}
