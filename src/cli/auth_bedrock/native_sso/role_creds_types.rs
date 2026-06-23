//! Wire types + conversion for `GetRoleCredentials` responses.

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;

use super::super::exported::Exported;
use crate::provider::bedrock::AwsCredentials;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RoleCredsEnvelope {
    pub role_credentials: RoleCreds,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RoleCreds {
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
    /// Expiration in epoch milliseconds.
    expiration: i64,
}

/// Convert raw SSO role credentials into the shared [`Exported`] shape.
pub(super) fn to_exported(rc: RoleCreds) -> Exported {
    let expiration: Option<DateTime<Utc>> = Utc.timestamp_millis_opt(rc.expiration).single();
    Exported {
        creds: AwsCredentials {
            access_key_id: rc.access_key_id,
            secret_access_key: rc.secret_access_key,
            session_token: Some(rc.session_token),
        },
        expiration,
    }
}
