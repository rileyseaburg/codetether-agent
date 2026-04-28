//! Kubernetes tool: manage K8s resources from the agent.
//!
//! Exposes cluster introspection, pod/deployment management, scaling,
//! rolling restarts, sub-agent pod lifecycle, and log retrieval through
//! a single `kubernetes` tool with an `action` parameter.

use super::{Tool, ToolResult};
use crate::k8s::K8sManager;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

#[derive(Debug, Clone, Deserialize)]
struct K8sInput {
    action: K8sAction,
    #[serde(default)]
    namespace: Option<String>,
    #[serde(default)]
    deployment: Option<String>,
    #[serde(default)]
    replicas: Option<i32>,
    #[serde(default)]
    subagent_id: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    env_vars: Option<HashMap<String, String>>,
    #[serde(default)]
    labels: Option<HashMap<String, String>>,
    #[serde(default)]
    command: Option<Vec<String>>,
    #[serde(default)]
    args: Option<Vec<String>>,
    #[serde(default = "default_tail_lines")]
    tail_lines: i64,
    #[serde(default)]
    label_selector: Option<String>,
}

fn default_tail_lines() -> i64 {
    100
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum K8sAction {
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

pub struct K8sTool {
    manager: Arc<OnceCell<K8sManager>>,
}

impl K8sTool {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(OnceCell::new()),
        }
    }

    /// Create a K8sTool with a pre-initialized manager.
    pub fn with_manager(manager: K8sManager) -> Self {
        let cell = OnceCell::new();
        let _ = cell.set(manager);
        Self {
            manager: Arc::new(cell),
        }
    }

    async fn get_manager(&self) -> &K8sManager {
        self.manager
            .get_or_init(|| async { K8sManager::new().await })
            .await
    }
}

impl Default for K8sTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for K8sTool {
    fn id(&self) -> &str {
        "kubernetes"
    }
    fn name(&self) -> &str {
        "kubernetes"
    }

    fn description(&self) -> &str {
        "Manage Kubernetes resources: check cluster status, list pods, scale deployments, trigger rolling restarts, spawn/delete sub-agent pods, fetch pod logs, and view recent management actions. Works both in-cluster and with local kubeconfig."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["status","list_pods","scale","rolling_restart",
                              "spawn_pod","delete_pod","pod_state","logs","recent_actions"],
                    "description": "The Kubernetes action to perform."
                },
                "namespace": {
                    "type": "string",
                    "description": "Namespace override for status, list_pods, scale, rolling_restart, spawn_pod, delete_pod, pod_state, and logs. Ignored by recent_actions."
                },
                "deployment": {
                    "type": "string",
                    "description": "Deployment override for status, scale, rolling_restart, and list_pods only. Ignored by pod and recent_actions operations."
                },
                "replicas": {
                    "type": "integer",
                    "description": "Number of replicas (required for scale)."
                },
                "subagent_id": {
                    "type": "string",
                    "description": "Sub-agent identifier (for spawn_pod, delete_pod, pod_state, logs)."
                },
                "image": {
                    "type": "string",
                    "description": "Container image for spawned sub-agent pods."
                },
                "env_vars": {
                    "type": "object",
                    "additionalProperties": {"type": "string"},
                    "description": "Environment variables for spawned sub-agent pods."
                },
                "labels": {
                    "type": "object",
                    "additionalProperties": {"type": "string"},
                    "description": "Labels for spawned sub-agent pods."
                },
                "command": {
                    "type": "array", "items": {"type": "string"},
                    "description": "Command override for spawned sub-agent pods."
                },
                "args": {
                    "type": "array", "items": {"type": "string"},
                    "description": "Args override for spawned sub-agent pods."
                },
                "tail_lines": {
                    "type": "integer",
                    "description": "Number of tail lines for logs (default: 100).",
                    "default": 100
                },
                "label_selector": {
                    "type": "string",
                    "description": "Label selector for list_pods (e.g. 'app=myapp')."
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let params: K8sInput = serde_json::from_value(input)?;
        let manager = self.get_manager().await;

        if !manager.is_available() {
            return Ok(ToolResult::error(
                "Kubernetes is not available. Ensure the agent is running inside a cluster or KUBECONFIG is set.",
            ));
        }

        match params.action.clone() {
            K8sAction::Status => Self::exec_status(manager, params).await,
            K8sAction::ListPods => Self::exec_list_pods(manager, params).await,
            K8sAction::Scale => Self::exec_scale(manager, params).await,
            K8sAction::RollingRestart => Self::exec_rolling_restart(manager, params).await,
            K8sAction::SpawnPod => Self::exec_spawn_pod(manager, params).await,
            K8sAction::DeletePod => Self::exec_delete_pod(manager, params).await,
            K8sAction::PodState => Self::exec_pod_state(manager, params).await,
            K8sAction::Logs => Self::exec_logs(manager, params).await,
            K8sAction::RecentActions => Self::exec_recent_actions(manager).await,
        }
    }
}

// ---------------------------------------------------------------------------
// Action handlers
// ---------------------------------------------------------------------------

impl K8sTool {
    async fn exec_status(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let status = target.status().await;
        let output = serde_json::to_string_pretty(&json!({
            "in_cluster": status.in_cluster,
            "namespace": status.namespace,
            "pod_name": status.pod_name,
            "deployment_name": status.deployment_name,
            "replicas": status.replicas,
            "available_replicas": status.available_replicas,
        }))?;
        Ok(ToolResult::success(output))
    }

    async fn exec_list_pods(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let pods = target
            .list_pods_with_selector(params.label_selector.as_deref())
            .await?;
        let pods_json: Vec<Value> = pods
            .iter()
            .map(|p| {
                json!({
                    "name": p.name,
                    "phase": p.phase,
                    "ready": p.ready,
                    "start_time": p.start_time,
                })
            })
            .collect();

        let output = json!({
            "namespace": target.status().await.namespace,
            "pods": pods_json,
            "count": pods_json.len(),
        });
        Ok(ToolResult::success(serde_json::to_string_pretty(&output)?))
    }

    async fn exec_scale(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let replicas = params
            .replicas
            .ok_or_else(|| anyhow!("'replicas' is required for the scale action"))?;
        let action = target.scale(replicas).await?;
        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "action": action.action,
            "success": action.success,
            "message": action.message,
            "timestamp": action.timestamp,
        }))?))
    }

    async fn exec_rolling_restart(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let action = target.rolling_restart().await?;
        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "action": action.action,
            "success": action.success,
            "message": action.message,
            "timestamp": action.timestamp,
        }))?))
    }

    async fn exec_spawn_pod(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let subagent_id = params
            .subagent_id
            .as_deref()
            .ok_or_else(|| anyhow!("'subagent_id' is required for the spawn_pod action"))?;

        let spec = crate::k8s::SubagentPodSpec {
            image: params.image,
            env_vars: params.env_vars.unwrap_or_default(),
            labels: params.labels.unwrap_or_default(),
            command: params.command,
            args: params.args,
        };

        let action = target
            .spawn_subagent_pod_with_spec(subagent_id, spec)
            .await?;
        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "action": action.action,
            "success": action.success,
            "message": action.message,
            "timestamp": action.timestamp,
        }))?))
    }

    async fn exec_delete_pod(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let subagent_id = params
            .subagent_id
            .as_deref()
            .ok_or_else(|| anyhow!("'subagent_id' is required for the delete_pod action"))?;
        let action = target.delete_subagent_pod(subagent_id).await?;
        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "action": action.action,
            "success": action.success,
            "message": action.message,
            "timestamp": action.timestamp,
        }))?))
    }

    async fn exec_pod_state(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let subagent_id = params
            .subagent_id
            .as_deref()
            .ok_or_else(|| anyhow!("'subagent_id' is required for the pod_state action"))?;

        let state = target
            .get_subagent_pod_state(subagent_id)
            .await?
            .ok_or_else(|| anyhow!("Pod for sub-agent '{}' not found", subagent_id))?;

        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "pod_name": state.pod_name,
            "phase": state.phase,
            "ready": state.ready,
            "terminated": state.terminated,
            "exit_code": state.exit_code,
            "reason": state.reason,
            "restart_count": state.restart_count,
        }))?))
    }

    async fn exec_logs(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let subagent_id = params
            .subagent_id
            .as_deref()
            .ok_or_else(|| anyhow!("'subagent_id' is required for the logs action"))?;
        let logs = target.subagent_logs(subagent_id, params.tail_lines).await?;
        Ok(ToolResult::success(logs))
    }

    async fn exec_recent_actions(manager: &K8sManager) -> Result<ToolResult> {
        let actions = manager.recent_actions(20).await;
        let actions_json: Vec<Value> = actions
            .iter()
            .map(|a| {
                json!({
                    "action": a.action,
                    "success": a.success,
                    "message": a.message,
                    "timestamp": a.timestamp,
                })
            })
            .collect();

        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "actions": actions_json,
            "count": actions_json.len(),
        }))?))
    }
}
