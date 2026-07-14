//! Kubernetes pod specification for a remote subtask.

use super::super::kubernetes_executor::SWARM_SUBTASK_PAYLOAD_ENV;
use crate::k8s::SubagentPodSpec;
use std::collections::HashMap;

const PASSTHROUGH: &[&str] = &[
    "VAULT_ADDR",
    "VAULT_TOKEN",
    "VAULT_MOUNT",
    "VAULT_SECRETS_PATH",
    "VAULT_NAMESPACE",
    "CODETETHER_AUTH_TOKEN",
];

pub(super) fn build(
    payload: String,
    swarm: &str,
    stage: usize,
    image: Option<String>,
) -> SubagentPodSpec {
    let mut env_vars = HashMap::from([(SWARM_SUBTASK_PAYLOAD_ENV.to_string(), payload)]);
    for key in PASSTHROUGH {
        if let Ok(value) = std::env::var(key)
            && !value.trim().is_empty()
        {
            env_vars.insert((*key).into(), value);
        }
    }
    let labels = HashMap::from([
        ("codetether.run/swarm-id".into(), swarm.into()),
        ("codetether.run/stage".into(), stage.to_string()),
    ]);
    SubagentPodSpec {
        image,
        env_vars,
        labels,
        command: Some(vec!["sh".into(), "-lc".into()]),
        args: Some(vec![format!(
            "exec codetether swarm-subagent --payload-env {SWARM_SUBTASK_PAYLOAD_ENV}"
        )]),
    }
}
