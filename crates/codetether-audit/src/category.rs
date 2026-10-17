//! Categories used to classify and filter audit events.

use serde::{Deserialize, Serialize};

/// Categories of auditable actions, serialized as snake_case strings.
///
/// # Examples
/// ```
/// use codetether_audit::AuditCategory;
/// let category = AuditCategory::Api;
/// assert_eq!(serde_json::to_string(&category).unwrap(), "\"api\"");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditCategory {
    /// HTTP API request/response.
    Api,
    /// Tool invocation.
    ToolExecution,
    /// Session lifecycle (create, load, prompt).
    Session,
    /// Cognition loop events.
    Cognition,
    /// Swarm persona operations.
    Swarm,
    /// Authentication events.
    Auth,
    /// Kubernetes self-deployment actions.
    K8s,
    /// Plugin sandbox events.
    Sandbox,
    /// Configuration changes.
    Config,
}
