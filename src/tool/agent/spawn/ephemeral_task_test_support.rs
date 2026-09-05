//! Request fixture for ephemeral workspace-policy checks.

use crate::tool::agent::params::Params;
use serde_json::json;
use std::path::Path;

pub(super) fn params(workspace: &Path, instructions: &str) -> Params {
    serde_json::from_value(json!({
        "action": "spawn", "name": "policy-task", "model": "test/model",
        "instructions": instructions, "ephemeral": true, "fork_turns": "none",
        "__ct_parent_workspace": workspace, "__ct_prior_context_allowed": false
    }))
    .unwrap()
}
