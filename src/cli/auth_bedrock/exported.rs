//! Parsing for `aws configure export-credentials --format process` output.

use crate::provider::bedrock::AwsCredentials;
use anyhow::{Context, Result};
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
}

/// Parse exported-credentials JSON into [`AwsCredentials`].
pub(super) fn parse(stdout: &[u8]) -> Result<AwsCredentials> {
    let parsed: ExportedCreds =
        serde_json::from_slice(stdout).context("Unexpected export-credentials JSON")?;
    Ok(AwsCredentials {
        access_key_id: parsed.access_key_id,
        secret_access_key: parsed.secret_access_key,
        session_token: parsed.session_token,
    })
}
