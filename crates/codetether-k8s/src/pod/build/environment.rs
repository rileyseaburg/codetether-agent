//! Sub-agent container environment entries.

use crate::SubagentPodSpec;
pub(super) fn entries(
    spec: &SubagentPodSpec,
    subagent_id: &str,
    namespace: &str,
) -> Vec<serde_json::Value> {
    let mut env_list: Vec<serde_json::Value> = spec
        .env_vars
        .iter()
        .map(|(k, v)| serde_json::json!({ "name": k, "value": v }))
        .collect();
    env_list.push(serde_json::json!({ "name": "CODETETHER_SUBAGENT_ID", "value": subagent_id }));
    env_list.push(serde_json::json!({ "name": "CODETETHER_K8S_NAMESPACE", "value": namespace }));

    env_list
}
