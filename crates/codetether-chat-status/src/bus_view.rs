//! Values used to render the current agent-bus status.
/// Snapshot of bus connectivity and work counts.
///
/// # Examples
/// ```
/// let status = codetether_chat_status::BusStatus::default();
/// assert!(!status.connected);
/// ```
#[derive(Default, Clone, Copy)]
pub struct BusStatus {
    /// Whether the remote bus is connected.
    pub connected: bool,
    /// Registered worker agents.
    pub agents: usize,
    /// Queued worker tasks.
    pub queued: usize,
    /// Effective bridge or local processing flag.
    pub processing: bool,
    /// Whether the local peer endpoint is ready.
    pub peer_ready: bool,
}
