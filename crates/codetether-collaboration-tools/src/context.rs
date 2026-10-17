//! Trusted runtime context shared across collaboration operations.
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::path::PathBuf;

#[derive(Default, Deserialize)]
/// Trusted runtime values preserved when forwarding agent operations.
///
/// # Examples
/// ```
/// let context=codetether_collaboration_tools::RuntimeContext::default();
/// assert!(context.session_id.is_none());
/// ```
pub struct RuntimeContext {
    #[serde(default, rename = "__ct_current_model")]
    /// Model injected by the parent runtime.
    pub model: Option<String>,
    #[serde(default, rename = "__ct_parent_workspace")]
    /// Workspace injected by the parent runtime.
    pub workspace: Option<PathBuf>,
    #[serde(default, rename = "__ct_session_id")]
    /// Owning parent session.
    pub session_id: Option<String>,
    #[serde(default, rename = "__ct_prior_context_allowed")]
    /// Whether the parent permits inherited context.
    pub prior_context_allowed: Option<bool>,
}
impl RuntimeContext {
    /// Return the optional parent workspace.
    pub fn workspace(&self) -> Option<&std::path::Path> {
        self.workspace.as_deref()
    }

    /// Insert each present trusted runtime value into the agent payload.
    pub fn inject(&self, payload: &mut Map<String, Value>) {
        if let Some(value) = &self.model {
            payload.insert("__ct_current_model".into(), json!(value));
        }
        if let Some(value) = &self.workspace {
            payload.insert("__ct_parent_workspace".into(), json!(value));
        }
        if let Some(value) = &self.session_id {
            payload.insert("__ct_session_id".into(), json!(value));
        }
        if let Some(value) = self.prior_context_allowed {
            payload.insert("__ct_prior_context_allowed".into(), json!(value));
        }
    }
}
