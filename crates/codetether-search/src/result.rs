//! Normalized result type produced by the router and consumed by the CLI.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::types::Backend;

/// One successful or failed backend execution.
///
/// # Examples
/// ```
/// use codetether_search::{Backend, BackendRun};
/// let run = BackendRun {
///     backend: Backend::Grep, success: true, output: "found".into(),
///     metadata: serde_json::Value::Null,
/// };
/// assert!(run.success);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendRun {
    /// Executed backend.
    pub backend: Backend,
    /// Whether the backend reported success.
    pub success: bool,
    /// Backend's rendered output.
    pub output: String,
    /// Additional structured backend metadata.
    #[serde(default)]
    pub metadata: Value,
}

/// Full router result returned to CLI / tool callers.
///
/// # Examples
/// ```
/// use codetether_search::RouterResult;
/// let result = RouterResult { query: "q".into(), router_model: "p/m".into(), runs: vec![] };
/// assert_eq!(result.success_count(), 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterResult {
    /// Original query.
    pub query: String,
    /// Resolved provider/model identifier.
    pub router_model: String,
    /// Backend results in selected execution order.
    pub runs: Vec<BackendRun>,
}

impl RouterResult {
    /// Shortcut for tests and callers: how many backends succeeded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_search::result::{BackendRun, RouterResult};
    /// use codetether_search::types::Backend;
    /// use serde_json::Value;
    /// let r = RouterResult {
    ///     query: "q".into(),
    ///     router_model: "zai/glm-5.1".into(),
    ///     runs: vec![BackendRun { backend: Backend::Grep, success: true, output: "ok".into(), metadata: Value::Null }],
    /// };
    /// assert_eq!(r.success_count(), 1);
    /// ```
    pub fn success_count(&self) -> usize {
        self.runs.iter().filter(|run| run.success).count()
    }
}
