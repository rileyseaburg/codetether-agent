//! Device authorization github_types responsibilities.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Decoded device authorization response used by the corresponding request or polling function.
///
/// # Examples
/// ```no_run
/// # fn decode(value: serde_json::Value) -> serde_json::Result<()> {
/// let response: codetether_device_auth::DeviceCodeResponse = serde_json::from_value(value)?;
/// # Ok(()) }
/// ```
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    #[serde(default)]
    pub verification_uri_complete: Option<String>,
    #[serde(default)]
    pub interval: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub error_description: Option<String>,
    #[serde(default)]
    pub interval: Option<u64>,
}
