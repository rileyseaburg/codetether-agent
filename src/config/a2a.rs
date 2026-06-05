use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A2A worker settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct A2aConfig {
    pub server_url: Option<String>,
    pub worker_name: Option<String>,
    #[serde(default)]
    pub auto_approve: AutoApprovePolicy,
    #[serde(default)]
    pub workspaces: Vec<PathBuf>,
}

/// A2A auto-approval policy.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AutoApprovePolicy {
    All,
    #[default]
    Safe,
    None,
}
