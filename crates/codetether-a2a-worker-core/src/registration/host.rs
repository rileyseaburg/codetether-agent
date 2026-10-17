//! Worker registration host projection.

pub fn host_metadata() -> (String, Option<String>) {
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    let k8s_node_name = std::env::var("K8S_NODE_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    (hostname, k8s_node_name)
}
