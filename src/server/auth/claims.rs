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
}
