//! Public user and resource context for authorization requests.
use serde::Serialize;

// ─── Input / Output types ────────────────────────────────────────

/// User context passed into the OPA input document.
#[derive(Debug, Clone, Serialize)]
/// # Examples
/// ```
/// let user = codetether_authorization_policy::PolicyUser {
///     user_id: "user".into(), roles: vec!["viewer".into()], tenant_id: None,
///     scopes: vec![], auth_source: "keycloak".into(),
/// };
/// assert_eq!(user.roles.len(), 1);
/// ```
pub struct PolicyUser {
    /// Authenticated user identifier.
    pub user_id: String,
    /// Assigned authorization roles.
    pub roles: Vec<String>,
    /// Optional tenant identifier.
    pub tenant_id: Option<String>,
    /// API-key permission scopes.
    pub scopes: Vec<String>,
    /// Authentication source, including api_key for scope enforcement.
    pub auth_source: String,
}

/// Resource context (optional).
#[derive(Debug, Clone, Default, Serialize)]
/// # Examples
/// ```
/// let resource = codetether_authorization_policy::PolicyResource::default();
/// assert!(resource.id.is_none());
/// ```
pub struct PolicyResource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    /// Optional resource category.
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional resource identifier.
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional resource owner identifier.
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional tenant identifier.
    pub tenant_id: Option<String>,
}
