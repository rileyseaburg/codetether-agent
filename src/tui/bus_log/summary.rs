//! Protocol summary data shown above the bus log.

/// Runtime protocol status rendered in the bus log summary panel.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::bus_log::ProtocolSummary;
///
/// let summary = ProtocolSummary {
///     cwd_display: ".".to_string(),
///     worker_id: None,
///     worker_name: None,
///     a2a_connected: false,
///     processing: None,
///     registered_agents: Vec::new(),
///     queued_tasks: 0,
///     recent_task: None,
///     peer_endpoint_ready: false,
/// };
/// assert!(!summary.a2a_connected);
/// ```
#[derive(Debug, Clone)]
pub struct ProtocolSummary {
    /// Display form of the active workspace path.
    pub cwd_display: String,
    /// Remote worker identifier, when known.
    pub worker_id: Option<String>,
    /// Human-readable worker name, when known.
    pub worker_name: Option<String>,
    /// Whether the A2A worker bridge is connected.
    pub a2a_connected: bool,
    /// Current remote worker processing state.
    pub processing: Option<bool>,
    /// Agents registered with the remote worker.
    pub registered_agents: Vec<String>,
    /// Number of queued remote tasks.
    pub queued_tasks: usize,
    /// Most recent task summary.
    pub recent_task: Option<String>,
    /// Whether the local A2A peer endpoint is ready.
    pub peer_endpoint_ready: bool,
}
