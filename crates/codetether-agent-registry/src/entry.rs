//! Metadata and durable identity for one open child thread.

/// A cloneable session with a durable identity.
/// # Implementors
/// Application session types implement this trait without changing persistence.
pub trait Session: Clone {
    /// Return the durable session ID used as the registry key.
    fn id(&self) -> &str;
}

/// One live agent's metadata and application session.
/// # Examples
/// ```rust
/// use codetether_agent_registry::{AgentEntry, Session};
/// #[derive(Clone)]
/// struct LocalSession(String);
/// impl Session for LocalSession { fn id(&self) -> &str { &self.0 } }
/// let entry = AgentEntry {
///     name: "reviewer".into(), instructions: "Review changes".into(),
///     session: LocalSession("child-1".into()), parent: None,
///     owner_session_id: Some("root".into()), depth: 1, model_id: None,
/// };
/// assert_eq!(entry.id(), "child-1");
/// ```
#[derive(Clone)]
pub struct AgentEntry<S> {
    /// Display name, which can repeat in different owner scopes.
    pub name: String,
    /// Instructions supplied when the agent was created.
    pub instructions: String,
    /// Live application session.
    pub session: S,
    /// Display name of the parent agent, when present.
    pub parent: Option<String>,
    /// Durable identity of the owning session.
    pub owner_session_id: Option<String>,
    /// Agent depth in its creation lineage.
    pub depth: u8,
    /// Model override, when specified.
    pub model_id: Option<String>,
}
impl<S: Session> AgentEntry<S> {
    /// Return this entry's durable session identity.
    pub fn id(&self) -> &str {
        self.session.id()
    }
}
