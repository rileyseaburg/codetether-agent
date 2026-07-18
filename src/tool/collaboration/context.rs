//! Trusted runtime context forwarded to the legacy agent engine.

use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::path::PathBuf;

#[derive(Default, Deserialize)]
pub(super) struct RuntimeContext {
    #[serde(default, rename = "__ct_current_model")]
    model: Option<String>,
    #[serde(default, rename = "__ct_parent_workspace")]
    workspace: Option<PathBuf>,
    #[serde(default, rename = "__ct_session_id")]
    pub session_id: Option<String>,
    #[serde(default, rename = "__ct_prior_context_allowed")]
    prior_context_allowed: Option<bool>,
}

impl RuntimeContext {
    pub(super) fn resume_config(&self) -> crate::tool::agent::residency::ResumeConfig {
        crate::tool::agent::residency::ResumeConfig::new(
            self.model.clone(),
            self.workspace.clone(),
            self.prior_context_allowed,
        )
    }

    pub(super) fn inject(&self, payload: &mut Map<String, Value>) {
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
