use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Permission rule collection.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionConfig {
    #[serde(default)]
    pub rules: HashMap<String, PermissionAction>,
    #[serde(default)]
    pub tools: HashMap<String, PermissionAction>,
    #[serde(default)]
    pub paths: HashMap<String, PermissionAction>,
}

/// Permission decision for a configured action.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PermissionAction {
    Allow,
    Deny,
    #[default]
    Ask,
}
