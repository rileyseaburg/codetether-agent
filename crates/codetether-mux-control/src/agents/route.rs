use serde_json::Value;

/// Match a mux roster entry by stable route ID.
///
/// # Arguments
/// * `item` - Projected agent-roster entry.
/// * `name` - Requested mux session route ID.
/// # Returns
/// Whether the entry's agent ID equals the requested route.
/// # Examples
/// ```
/// let item = serde_json::json!({"agent_id":"route", "name":"Display"});
/// assert!(codetether_mux_control::is_agent_route(&item, "route"));
/// assert!(!codetether_mux_control::is_agent_route(&item, "Display"));
/// ```
pub fn is_agent_route(item: &Value, name: &str) -> bool {
    item["agent_id"] == name
}

#[cfg(test)]
#[path = "route_tests.rs"]
mod tests;
