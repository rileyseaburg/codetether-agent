//! Advertised HTTP and bus endpoint URLs.

/// Builds the advertised HTTP and bus interface URLs for this worker.
///
/// # Examples
/// ```
/// use codetether_a2a_worker_core::{advertised_interfaces};
/// assert_eq!(advertised_interfaces(None),serde_json::json!({}));
/// ```
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
