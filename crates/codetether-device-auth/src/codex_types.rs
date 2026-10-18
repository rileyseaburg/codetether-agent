//! Device authorization codex_types responsibilities.
use crate::interval::deserialize_interval_seconds;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Decoded device authorization response used by the corresponding request or polling function.
///
/// # Examples
/// ```no_run
/// # fn decode(value: serde_json::Value) -> serde_json::Result<()> {
/// let response: codetether_device_auth::CodexDeviceCodeResponse = serde_json::from_value(value)?;
/// # Ok(()) }
/// ```
pub struct CodexDeviceCodeResponse {
    pub device_auth_id: String,
    #[serde(alias = "usercode")]
    pub user_code: String,
    #[serde(default, deserialize_with = "deserialize_interval_seconds")]
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
/// Decoded device authorization response used by the corresponding request or polling function.
///
/// # Examples
/// ```no_run
/// # fn decode(value: serde_json::Value) -> serde_json::Result<()> {
/// let response: codetether_device_auth::CodexDeviceCodeTokenResponse = serde_json::from_value(value)?;
/// # Ok(()) }
/// ```
pub struct CodexDeviceCodeTokenResponse {
    pub authorization_code: String,
    pub code_verifier: String,
}

#[derive(Debug, Deserialize)]
pub struct CodexDeviceErrorResponse {
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub error_description: Option<String>,
}
