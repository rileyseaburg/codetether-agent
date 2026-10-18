/// Borrowed tool-call identity and parent-turn correlation.
/// # Examples
/// ```
/// let context = codetether_bus_core::tool_events::Context {
///     agent: "build", turn_id: "turn-1", step: 1,
///     request_id: "call-1", tool_name: "read",
/// };
/// assert_eq!(context.step, 1);
/// ```
pub struct Context<'a> {
    /// Session's agent identity used for topics and payloads.
    pub agent: &'a str,
    /// Parent turn correlation identifier.
    pub turn_id: &'a str,
    /// Current provider step.
    pub step: usize,
    /// Provider-assigned tool-call identifier.
    pub request_id: &'a str,
    /// Canonical tool name.
    pub tool_name: &'a str,
}
