//! SubagentPodSpec data contract.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kubernetes pod launch options for a sub-agent.
///
/// # Examples
/// ```
/// use codetether_k8s::{SubagentPodSpec};
/// let spec = SubagentPodSpec { image: Some("example/agent:v1".into()),
///     ..SubagentPodSpec::default() };
/// assert_eq!(spec.image.as_deref(), Some("example/agent:v1"));
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubagentPodSpec {
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub env_vars: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    pub args: Option<Vec<String>>,
}
