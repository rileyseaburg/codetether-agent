//! Worker capability and interface helpers.
//!
//! This module keeps capability advertisement and public endpoint metadata out
//! of the main worker loop so connection setup stays focused on orchestration.
//!
//! # Examples
//!
//! ```ignore
//! let caps = worker_capabilities();
//! assert!(caps.iter().any(|cap| cap == "a2a"));
//! ```

/// Default A2A control-plane URL used by worker callers.
///
/// This constant gives CLI defaults and tests a stable endpoint value when no
/// explicit server URL is provided.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(DEFAULT_A2A_SERVER_URL, "https://api.codetether.run");
/// ```
pub const DEFAULT_A2A_SERVER_URL: &str = "https://api.codetether.run";

const BASE_WORKER_CAPABILITIES: &[&str] = &[
    "forage", "ralph", "swarm", "rlm", "a2a", "mcp", "grpc", "grpc-web", "jsonrpc",
];

/// Returns the capability list advertised during worker registration.
///
/// Knative-backed workers append a `knative` capability so the control plane
/// can route tasks that require serverless execution.
///
/// # Examples
///
/// ```ignore
/// let caps = worker_capabilities();
/// assert!(!caps.is_empty());
/// ```
pub(super) fn worker_capabilities() -> Vec<String> {
    let mut capabilities = BASE_WORKER_CAPABILITIES
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let is_knative = std::env::var("KNATIVE_SERVICE")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(false);
    if is_knative {
        capabilities.push("knative".to_string());
    }
    capabilities
}

/// Builds the advertised HTTP and bus interface URLs for this worker.
///
/// Empty or missing public URLs produce an empty object so callers can omit
/// interfaces safely when the worker is not externally reachable.
///
/// # Examples
///
/// ```ignore
/// let value = advertised_interfaces(Some("https://worker.example"));
/// assert!(value.get("http").is_some());
/// ```
pub(super) fn advertised_interfaces(public_url: Option<&str>) -> serde_json::Value {
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
