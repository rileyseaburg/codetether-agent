//! Live child identity required when publishing lifecycle events.
/// A live child snapshot used by lifecycle publication.
/// # Examples
/// ```rust
/// use codetether_agent_status::PublishedAgent;
/// let child = PublishedAgent { id: "child".into(), name: "reviewer".into(),
///     owner_session_id: Some("parent".into()) };
/// assert_eq!(child.name, "reviewer");
/// ```
pub struct PublishedAgent {
    /// Durable child session ID used in task topics.
    pub id: String,
    /// Display name included in parent-facing results.
    pub name: String,
    /// Optional owning session that receives the result.
    pub owner_session_id: Option<String>,
}
/// Supply an existing child identity without creating a new entry.
/// # Implementors
/// Applications forward this lookup to the current live child registry.
pub trait PublishHost {
    /// Clone the live child's publication fields, or return none when absent.
    fn get(agent_id: &str) -> Option<PublishedAgent>;
}
