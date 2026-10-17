//! Base pod labels and explicit label overrides.

use crate::SubagentPodSpec;

fn sanitize_label_value(input: &str) -> String {
    let mut value: String = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .take(63)
        .collect();
    value = value
        .trim_matches(|c| c == '-' || c == '_' || c == '.')
        .to_string();
    if value.is_empty() {
        "subagent".to_string()
    } else {
        value
    }
}
pub(super) fn labels(spec: &SubagentPodSpec, subagent_id: &str) -> serde_json::Value {
    let mut labels = serde_json::json!({
        "app": "codetether",
        "codetether.run/role": "subagent",
        "codetether.run/subagent-id": sanitize_label_value(subagent_id)
    });

    if let Some(map) = labels.as_object_mut() {
        for (k, v) in &spec.labels {
            map.insert(k.clone(), serde_json::json!(v));
        }
    }

    labels
}
