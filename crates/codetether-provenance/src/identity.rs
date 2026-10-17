//! Git author identity formatting.

mod environment;
mod sanitize;
pub use environment::git_identity_env_vars;
use sanitize::sanitize_identity;
const GIT_EMAIL_DOMAIN: &str = "codetether.run";

const GIT_NAME: &str = "CodeTether Agent";

///
/// # Examples
/// ```
/// use codetether_provenance::{git_author_name};
/// assert_eq!(git_author_name(Some("User:ABC")),"CodeTether Agent [user-abc]");
/// ```
pub fn git_author_name(agent_identity_id: Option<&str>) -> String {
    match agent_identity_id.and_then(sanitize_identity) {
        Some(identity) => format!("{GIT_NAME} [{identity}]"),
        None => GIT_NAME.to_string(),
    }
}

///
/// # Examples
/// ```
/// use codetether_provenance::{git_author_email};
/// assert_eq!(git_author_email(None,Some("worker"),None),"agent+worker@codetether.run");
/// ```
pub fn git_author_email(
    agent_identity_id: Option<&str>,
    worker_id: Option<&str>,
    provenance_id: Option<&str>,
) -> String {
    let identity = [agent_identity_id, worker_id, provenance_id]
        .into_iter()
        .flatten()
        .find_map(sanitize_identity)
        .unwrap_or_else(|| "unknown".to_string());
    format!("agent+{identity}@{GIT_EMAIL_DOMAIN}")
}
#[cfg(test)]
mod tests;
