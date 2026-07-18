//! Versioned durable identity for one spawned child session.

use super::super::store::AgentEntry;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Lifecycle {
    #[default]
    Open,
    Closed,
}

#[derive(Clone, Deserialize, Serialize)]
pub(super) struct Manifest {
    pub version: u8,
    pub name: String,
    pub instructions: String,
    pub child_session_id: String,
    pub parent: Option<String>,
    pub owner_session_id: Option<String>,
    pub depth: u8,
    pub model_id: Option<String>,
    #[serde(default)]
    pub lifecycle: Lifecycle,
}

impl Manifest {
    pub(super) fn from_entry(name: &str, entry: &AgentEntry) -> Self {
        Self {
            version: 1,
            name: name.into(),
            instructions: entry.instructions.clone(),
            child_session_id: entry.session.id.clone(),
            parent: entry.parent.clone(),
            owner_session_id: entry.owner_session_id.clone(),
            depth: entry.depth,
            model_id: entry.model_id.clone(),
            lifecycle: Lifecycle::Open,
        }
    }

    pub(super) fn with_lifecycle(mut self, lifecycle: Lifecycle) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    pub(super) fn supports_current_schema(&self) -> bool {
        self.version == 1
    }
}
