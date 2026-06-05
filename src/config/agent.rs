use crate::config::PermissionAction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Per-agent runtime configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub prompt: Option<String>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub permissions: HashMap<String, PermissionAction>,
    #[serde(default)]
    pub disabled: bool,
}
