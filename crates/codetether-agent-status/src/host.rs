//! Application snapshots needed by owner-scoped child status reporting.
use crate::AgentView;
use std::collections::HashMap;
/// Live child snapshots and durable task-to-child ownership.
/// # Implementors
/// Applications delegate to their existing child registry and task-ID mapping.
pub trait Host {
    /// Return live children for the owner, or the existing all-owner view for none.
    fn agents(parent: Option<&str>) -> Vec<AgentView>;
    /// Return owned task IDs mapped to canonical child IDs.
    fn task_to_agent(parent: Option<&str>) -> HashMap<String, String>;
}
