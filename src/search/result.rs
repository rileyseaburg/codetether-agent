//! Normalized result type produced by the router and consumed by the CLI.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::types::Backend;

/// One successful or failed backend execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendRun {
    pub backend: Backend,
    pub success: bool,
    pub output: String,
    #[serde(default)]
    pub metadata: Value,
}

/// Full router result returned to CLI / tool callers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterResult {
    pub query: String,
    pub router_model: String,
    pub runs: Vec<BackendRun>,
}

impl RouterResult {
    /// Shortcut for tests and callers: how many backends succeeded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::search::result::{BackendRun, RouterResult};
    /// use codetether_agent::search::types::Backend;
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
