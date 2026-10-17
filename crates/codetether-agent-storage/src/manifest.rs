//! Versioned durable identity for one spawned child session.

use serde::{Deserialize, Serialize};

/// Persisted lifecycle marker for a durable child identity.
///
/// # Examples
/// ```
/// assert_eq!(codetether_agent_storage::Lifecycle::default(),codetether_agent_storage::Lifecycle::Open);
/// ```
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Lifecycle {
    #[default]
    /// Available for restoration.
    Open,
    /// Explicitly closed, but retained for later resume.
    Closed,
}

/// Versioned durable identity and parent linkage for a child.
///
/// # Examples
/// ```
/// let manifest=codetether_agent_storage::Manifest {
/// version:1,name:"child".into(),instructions:"work".into(),child_session_id:"id".into(),
/// parent:None,owner_session_id:None,depth:1,model_id:None,
/// lifecycle:codetether_agent_storage::Lifecycle::Open,
/// };
/// assert!(manifest.supports_current_schema());
/// ```
#[derive(Clone, Deserialize, Serialize)]
pub struct Manifest {
    /// Storage schema version.
    pub version: u8,
    /// Child name within its parent.
    pub name: String,
    /// Persistent child task instructions.
    pub instructions: String,
    /// Durable child session identity.
    pub child_session_id: String,
    /// Parent linkage recorded by the runtime.
    pub parent: Option<String>,
    /// Parent session owning this durable identity.
    pub owner_session_id: Option<String>,
    /// Recorded child nesting depth.
    pub depth: u8,
    /// Optional selected provider/model identity.
    pub model_id: Option<String>,
    #[serde(default)]
    /// Persisted lifecycle marker.
    pub lifecycle: Lifecycle,
}

impl Manifest {
    /// Return this manifest with the supplied lifecycle marker.
    pub fn with_lifecycle(mut self, lifecycle: Lifecycle) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    /// Report whether this is the supported version-one schema.
    pub fn supports_current_schema(&self) -> bool {
        self.version == 1
    }
}
