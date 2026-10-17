//! GitHub PR signature context.
use super::*;

#[derive(Clone, Debug)]
/// PR SignatureContext data.
///
/// # Examples
/// ```
/// let signature = codetether_github_pr::context::SignatureContext {
/// agent_identity_id: None, model: None, tenant_id: None, worker_id: None,
/// run_id: None, attempt_id: None, session_id: None, provenance_signature: None,
/// github_installation_id: None, github_app_id: None, head_sha: "sha".into(), workspace_id: "workspace".into(),
/// };
/// assert_eq!(signature.head_sha, "sha");
/// ```
pub struct SignatureContext {
    /// agent identity id metadata.
    pub agent_identity_id: Option<String>,
    /// model metadata.
    pub model: Option<String>,
    /// tenant id metadata.
    pub tenant_id: Option<String>,
    /// worker id metadata.
    pub worker_id: Option<String>,
    /// run id metadata.
    pub run_id: Option<String>,
    /// attempt id metadata.
    pub attempt_id: Option<String>,
    /// session id metadata.
    pub session_id: Option<String>,
    /// provenance signature metadata.
    pub provenance_signature: Option<String>,
    /// github installation id metadata.
    pub github_installation_id: Option<String>,
    /// github app id metadata.
    pub github_app_id: Option<String>,
    /// head sha metadata.
    pub head_sha: String,
    /// workspace id metadata.
    pub workspace_id: String,
}

impl SignatureContext {
    #[cfg(test)]
    pub fn sample() -> Self {
        Self {
            agent_identity_id: Some("user:test".to_string()),
            model: Some("zai/glm-5".to_string()),
            tenant_id: None,
            worker_id: None,
            run_id: None,
            attempt_id: None,
            session_id: None,
            provenance_signature: Some("sig".to_string()),
            github_installation_id: None,
            github_app_id: None,
            head_sha: "abc123".to_string(),
            workspace_id: "ws-1".to_string(),
        }
    }
}
