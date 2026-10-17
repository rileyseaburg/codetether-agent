//! input for Kubernetes tool execution.
use super::*;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct K8sInput {
    pub(crate) action: K8sAction,
    #[serde(default)]
    pub(crate) namespace: Option<String>,
    #[serde(default)]
    pub(crate) deployment: Option<String>,
    #[serde(default)]
    pub(crate) replicas: Option<i32>,
    #[serde(default)]
    pub(crate) subagent_id: Option<String>,
    #[serde(default)]
    pub(crate) image: Option<String>,
    #[serde(default)]
    pub(crate) env_vars: Option<HashMap<String, String>>,
    #[serde(default)]
    pub(crate) labels: Option<HashMap<String, String>>,
    #[serde(default)]
    pub(crate) command: Option<Vec<String>>,
    #[serde(default)]
    pub(crate) args: Option<Vec<String>>,
    #[serde(default = "default_tail_lines")]
    pub(crate) tail_lines: i64,
    #[serde(default)]
    pub(crate) label_selector: Option<String>,
}

fn default_tail_lines() -> i64 {
    100
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum K8sAction {
    Status,
    ListPods,
    Scale,
    RollingRestart,
    SpawnPod,
    DeletePod,
    PodState,
    Logs,
    RecentActions,
}
