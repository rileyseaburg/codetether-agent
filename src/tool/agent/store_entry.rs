//! Metadata and durable identity for one open child thread.

use crate::session::Session;

#[derive(Clone)]
pub(in crate::tool::agent) struct AgentEntry {
    pub name: String,
    pub instructions: String,
    pub session: Session,
    pub parent: Option<String>,
    pub owner_session_id: Option<String>,
    pub depth: u8,
    pub model_id: Option<String>,
}

impl AgentEntry {
    pub(in crate::tool::agent) fn id(&self) -> &str {
        &self.session.id
    }
}
