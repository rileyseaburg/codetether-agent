//! Safe error text for rejected auth requests.
//!
//! Vault puts its reason in `{"errors": [...]}` (e.g. "invalid audience (aud)
//! claim"). A reason is surfaced only if it echoes none of the request's
//! secrets and contains nothing that looks like a token; otherwise only the
//! HTTP status is reported, keeping credentials out of errors and logs.

use serde::Deserialize;

#[derive(Deserialize)]
struct VaultErrors {
    errors: Vec<String>,
}

/// Build the error message for a rejected request from its status and body.
///
/// `secrets` are values sent with the request (tokens, JWTs); a reason that
/// contains any of them is discarded.
pub(super) fn message(status: u16, body: &str, secrets: &[&str]) -> String {
    let reason = serde_json::from_str::<VaultErrors>(body)
        .ok()
        .and_then(|parsed| parsed.errors.into_iter().next())
        .map(|text| text.trim().chars().take(300).collect::<String>())
        .filter(|text| !text.is_empty() && safe(text, secrets));
    match reason {
        Some(reason) => format!("Authentication request rejected (HTTP {status}): {reason}"),
        None => format!("Authentication request rejected (HTTP {status})"),
    }
}

fn safe(text: &str, secrets: &[&str]) -> bool {
    let echoes = secrets
        .iter()
        .any(|secret| secret.len() >= 4 && text.contains(secret));
    let token_like = text
        .split(|c: char| c.is_whitespace() || c == '"' || c == '\'')
        .any(|word| word.starts_with("hvs.") || word.starts_with("eyJ") || word.len() >= 40);
    !echoes && !token_like
}
