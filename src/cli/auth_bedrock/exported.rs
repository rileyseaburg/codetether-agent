//! Parsing for `aws configure export-credentials --format process` output.

use crate::provider::bedrock::AwsCredentials;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// JSON shape of `aws configure export-credentials --format process`.
#[derive(Deserialize)]
struct ExportedCreds {
    #[serde(rename = "AccessKeyId")]
    access_key_id: String,
    #[serde(rename = "SecretAccessKey")]
    secret_access_key: String,
    #[serde(rename = "SessionToken")]
    session_token: Option<String>,
    #[serde(rename = "Expiration")]
    expiration: Option<String>,
}

/// Credentials plus the instant the underlying session expires, if known.
///
/// The expiration is the real lifetime ceiling for any minted Bedrock API
/// key: when the SSO/STS session dies, every token signed from it dies too,
/// regardless of the token's own `X-Amz-Expires`.
pub(super) struct Exported {
    pub creds: AwsCredentials,
    pub expiration: Option<DateTime<Utc>>,
}

/// Parse exported-credentials JSON into [`Exported`].
pub(super) fn parse(stdout: &[u8]) -> Result<Exported> {
    let parsed: ExportedCreds =
        serde_json::from_slice(stdout).context("Unexpected export-credentials JSON")?;
    let expiration = parsed
        .expiration
        .as_deref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    Ok(Exported {
        creds: AwsCredentials {
            access_key_id: parsed.access_key_id,
            secret_access_key: parsed.secret_access_key,
            session_token: parsed.session_token,
        },
        expiration,
    })
}
