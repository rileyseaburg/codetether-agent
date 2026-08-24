//! Signed parent network policy carried by agent tool parameters.

use serde::Deserialize;
use serde_json::json;
use std::path::Path;

#[derive(Default, Deserialize)]
pub(super) struct ParentPolicy {
    #[serde(default, rename = "__ct_effective_network_allowed")]
    network_allowed: Option<bool>,
    #[serde(default, rename = "__ct_network_authority")]
    network_authority: Option<String>,
}

impl ParentPolicy {
    pub(super) fn trusted(
        &self,
        session: Option<&str>,
        workspace: Option<&Path>,
    ) -> Option<bool> {
        let args = json!({
            "__ct_session_id":session?,
            "__ct_parent_workspace":workspace.map(Path::display).map(|path| path.to_string()),
            "__ct_effective_network_allowed":self.network_allowed?,
            "__ct_network_authority":self.network_authority,
        });
        crate::tool::network_access::trusted_value(&args)
    }
}

#[cfg(test)]
#[path = "params_policy_tests.rs"]
mod tests;

impl super::params::Params {
    pub(super) fn resume_config(&self) -> super::residency::ResumeConfig {
        super::residency::ResumeConfig::new(
            self._current_model.clone(),
            self.parent_workspace.clone(),
            self.parent_prior_context_allowed,
        )
        .with_network(self.parent_network_allowed())
    }

    pub(super) fn parent_network_allowed(&self) -> Option<bool> {
        self.parent_policy.trusted(
            self.parent_session_id.as_deref(),
            self.parent_workspace.as_deref(),
        )
    }
}