//! Actionable error mapping for Bedrock stream auth failures.
//!
//! A 401/403 from converse-stream almost always means the bearer token's
//! underlying SSO/STS session expired (these keys die with the session,
//! before their nominal 12h TTL). We surface the recovery command instead
//! of a bare HTTP status.

use reqwest::StatusCode;

/// Build a stream error message, appending refresh guidance on auth failures.
pub(super) fn message(status: StatusCode, body: &str) -> String {
    let base = format!("Bedrock stream error ({status}): {body}");
    if codetether_bedrock_protocol::audit::pairing_error::is_pairing_failure(body) {
        return codetether_bedrock_protocol::audit::pairing_error::guidance(&base);
    }
    if is_auth_failure(status) {
        format!(
            "{base}\n\
             The Bedrock API key was rejected — its SSO/STS session has likely \
             expired. Renew it without a browser by running:\n\
             \x20\x20codetether auth bedrock --refresh --save"
        )
    } else {
        base
    }
}

/// Whether the status indicates an authentication/authorization failure.
fn is_auth_failure(status: StatusCode) -> bool {
    matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN)
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
