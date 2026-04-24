use serde::{Deserialize, Serialize};

/// JWT claims extracted from a Bearer token payload.
///
/// These claims are attached to the request after authentication succeeds so
/// downstream handlers can scope behavior such as bus topic access.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::server::auth::JwtClaims;
///
/// let claims = JwtClaims {
///     topics: vec!["agent.alpha".into()],
///     subject: Some("worker-1".into()),
///     scopes: vec!["bus:read".into()],
///     roles: vec!["viewer".into()],
///     tenant_id: Some("tenant-1".into()),
///     auth_source: Some("jwt".into()),
/// };
///
/// assert_eq!(claims.subject.as_deref(), Some("worker-1"));
/// assert_eq!(claims.topics, vec!["agent.alpha"]);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Allowed topics for bus stream filtering.
    #[serde(default)]
    pub topics: Vec<String>,
    /// Optional user identifier.
    #[serde(default, rename = "sub")]
    pub subject: Option<String>,
    /// Additional scopes from the JWT.
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Authorization roles from the JWT or API key claim payload.
    #[serde(default)]
    pub roles: Vec<String>,
    /// Optional tenant identifier for policy isolation.
    #[serde(default)]
    pub tenant_id: Option<String>,
    /// Authentication source, for example `jwt` or `api_key`.
    #[serde(default)]
    pub auth_source: Option<String>,
}
