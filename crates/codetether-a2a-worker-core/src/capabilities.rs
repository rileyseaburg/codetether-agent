//! Worker capability and interface helpers.

/// Default A2A control-plane URL used by worker callers.
pub const DEFAULT_A2A_SERVER_URL: &str = "https://api.codetether.run";

const BASE_WORKER_CAPABILITIES: &[&str] = &[
    "forage", "ralph", "swarm", "rlm", "a2a", "mcp", "grpc", "grpc-web", "jsonrpc",
];

/// Returns the capability list advertised during worker registration.
pub fn worker_capabilities() -> Vec<String> {
    let mut capabilities = BASE_WORKER_CAPABILITIES
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    if is_knative() {
        capabilities.push("knative".to_string());
    }
    capabilities
}

/// Builds the advertised HTTP and bus interface URLs for this worker.
pub fn advertised_interfaces(public_url: Option<&str>) -> serde_json::Value {
    let Some(base_url) = public_url.map(str::trim).filter(|value| !value.is_empty()) else {
        return serde_json::json!({});
    };
    let base_url = base_url.trim_end_matches('/');
    serde_json::json!({
        "http": { "base_url": base_url },
        "bus": {
            "stream_url": format!("{base_url}/v1/bus/stream"),
            "publish_url": format!("{base_url}/v1/bus/publish"),
        },
    })
}

fn is_knative() -> bool {
    std::env::var("KNATIVE_SERVICE")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(false)
}
