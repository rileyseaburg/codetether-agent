//! OAuth discovery metadata received from the pinned Monarch origin.
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct ResourceMetadata {
    pub(super) resource: String,
    pub(super) authorization_servers: Vec<String>,
    pub(super) scopes_supported: Vec<String>,
}

#[derive(Deserialize)]
pub(super) struct ServerMetadata {
    pub(super) issuer: String,
    pub(super) authorization_endpoint: String,
    pub(super) token_endpoint: String,
    pub(super) registration_endpoint: String,
    pub(super) revocation_endpoint: String,
    pub(super) code_challenge_methods_supported: Vec<String>,
    pub(super) token_endpoint_auth_methods_supported: Vec<String>,
    pub(super) response_types_supported: Vec<String>,
    pub(super) grant_types_supported: Vec<String>,
}
