//! Registration response fields used for account completion output.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct RegisterResponse {
    pub(super) user_id: String,
    pub(super) email: String,
    pub(super) message: String,
    #[serde(default)]
    pub(super) instance_url: Option<String>,
    #[serde(default)]
    pub(super) instance_namespace: Option<String>,
    #[serde(default)]
    pub(super) provisioning_status: Option<String>,
}
