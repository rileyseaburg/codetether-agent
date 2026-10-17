//! Worker task agent-name normalization.
/// Preserve supported task agent modes, falling back to the build agent.
///
/// # Arguments
/// * `raw_agent` - Requested task agent mode.
/// # Returns
/// The original supported mode, or `build` for other values.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::preparation::normalize_agent;
/// assert_eq!(normalize_agent("plan"), "plan");
/// assert_eq!(normalize_agent("unknown"), "build");
/// ```

pub fn normalize_agent(raw_agent: &str) -> String {
    if crate::is_swarm_agent(raw_agent) || matches!(raw_agent, "build" | "plan") {
        return raw_agent.to_string();
    }
    tracing::info!(target: "codetether_agent::a2a::worker::task_session_resolve", agent = %raw_agent, "Falling back to build agent");
    "build".to_string()
}
