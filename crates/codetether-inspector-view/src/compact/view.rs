/// Borrowed session state displayed in the compact inspector.
///
/// # Examples
/// ```
/// let view = codetether_inspector_view::compact::View::default();
/// assert!(!view.processing);
/// ```
#[derive(Default)]
pub struct View<'a> {
    /// Whether a request is running.
    pub processing: bool,
    /// Tool currently pending, taking precedence over the last tool.
    pub pending_tool: Option<&'a str>,
    /// Most recently completed tool.
    pub last_tool: Option<&'a str>,
    /// Last completion's model label.
    pub model: Option<&'a str>,
    /// Current spawned agent, if any.
    pub active_agent: Option<&'a str>,
    /// Number of agents registered with the worker bridge.
    pub registered: usize,
    /// Current estimated context token usage.
    pub context_used: Option<usize>,
    /// Current context token budget.
    pub context_budget: Option<usize>,
    /// Remote chat synchronization status.
    pub sync: Option<&'a str>,
    /// Active session identifier.
    pub session_id: Option<&'a str>,
    /// Number of transcript messages.
    pub message_count: usize,
    /// Number of spawned agents.
    pub agent_count: usize,
}
