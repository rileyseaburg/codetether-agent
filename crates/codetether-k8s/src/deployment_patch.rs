//! Deployment mutation payload construction.

use std::collections::BTreeMap;

pub fn scale(replicas: i32, annotations: BTreeMap<String, String>) -> serde_json::Value {
    serde_json::json!({
        "metadata": { "annotations": annotations },
        "spec": { "replicas": replicas }
    })
}

pub fn restart(annotations: BTreeMap<String, String>) -> serde_json::Value {
    serde_json::json!({
        "spec": { "template": { "metadata": { "annotations": annotations } } }
    })
}
