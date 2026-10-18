//! Application-independent spawned-agent ownership and hierarchy.
/// A spawned sub-agent with its own independent session.
///
/// The session type is supplied by the application; tree and display code do
/// not require its runtime implementation.
///
/// # Examples
/// ```
/// let agent = codetether_agent_tree::SpawnedAgent {
///     name: "planner".into(), instructions: "Plan the work".into(),
///     parent: None, depth: 0, session: (), model_id: None, is_processing: false,
/// };
/// assert_eq!(agent.name, "planner");
/// ```
#[allow(dead_code)]
pub struct SpawnedAgent<S> {
    /// User-facing name (e.g. "planner", "coder")
    pub name: String,
    /// System instructions for this agent
    pub instructions: String,
    /// Name of the agent that spawned this one; `None` means the main chat
    /// (a top-level sibling). Forms the agent tree used by the header rail.
    pub parent: Option<String>,
    /// Nesting depth: 0 for children of main, up to
    /// [`crate::MAX_AGENT_DEPTH`].
    pub depth: u8,
    /// Independent conversation session
    pub session: S,
    /// Model selected or last used when this agent was created.
    pub model_id: Option<String>,
    /// Whether this agent is currently processing a message
    pub is_processing: bool,
}

impl<S> crate::HierarchyEntry for SpawnedAgent<S> {
    fn parent_name(&self) -> Option<&str> {
        self.parent.as_deref()
    }
    fn depth(&self) -> u8 {
        self.depth
    }
}
