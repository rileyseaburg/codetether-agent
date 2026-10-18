//! Kubernetes API request bodies.
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScaleRequest {
    pub replicas: i32,
}

#[derive(Deserialize)]
pub struct SpawnSubagentRequest {
    pub subagent_id: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub env_vars: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub labels: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    pub args: Option<Vec<String>>,
}
