//! Wire types for the AWS SSO-OIDC device authorization flow.
//!
//! These mirror the JSON returned by `RegisterClient`,
//! `StartDeviceAuthorization`, and `CreateToken` on the public
//! `oidc.<region>.amazonaws.com` endpoint (no SigV4 required).

use serde::Deserialize;

/// Response from `RegisterClient`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RegisteredClient {
    pub client_id: String,
    pub client_secret: String,
}

/// Response from `StartDeviceAuthorization`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DeviceAuthorization {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
    pub interval: Option<u64>,
}

/// Successful response from `CreateToken`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DeviceToken {
    pub access_token: String,
    pub expires_in: i64,
}
