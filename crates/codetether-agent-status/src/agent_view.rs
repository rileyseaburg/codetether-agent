//! Owned live-child fields required to render a status row.
/// Snapshot of one resident child before consulting lifecycle history.
/// # Examples
/// ```rust
/// use codetether_agent_status::AgentView;
/// let child = AgentView { name: "reviewer".into(), id: "child-id".into(), messages: 3 };
/// assert_eq!(child.messages, 3);
/// ```
pub struct AgentView {
    /// Child display name.
    pub name: String,
    /// Durable child session ID.
    pub id: String,
    /// Messages present in the live session snapshot.
    pub messages: usize,
}
