//! Parameter parsing types for the agent tool.
//!
//! This module defines the deserializable input payload shared by agent
//! tool actions. It keeps tool input concerns separate from runtime
//! behavior such as spawning, messaging, or persistence.
//!
//! # Examples
//!
//! ```ignore
//! let params: Params = serde_json::from_value(payload)?;
//! assert_eq!(params.action, "spawn");
//! ```

use serde::Deserialize;
use std::path::PathBuf;

use super::collaboration_runtime::message_input::MessageImage;

/// Parsed input payload for the sub-agent management tool.
///
/// This structure is shared by all agent-tool actions so parsing happens once
/// before action-specific validation.
///
/// # Examples
///
/// ```ignore
/// let params: Params = serde_json::from_value(payload)?;
/// ```
#[derive(Deserialize)]
pub(super) struct Params {
    pub action: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    /// Reusable A2A conversation identifier for a remote message.
    #[serde(default)]
    pub context_id: Option<String>,
    #[serde(default, rename = "__ct_message_images")]
    pub message_images: Vec<MessageImage>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub ephemeral: bool,
    /// Controls whether `message`/`spawn` blocks the caller.
    ///
    /// `None` (default) waits for the child result so model callers cannot
    /// finish before their delegated work. Interactive callers opt into detach.
    #[serde(default)]
    pub detach: Option<bool>,
    #[serde(default)]
    pub fork_turns: Option<String>,
    #[serde(default, rename = "__ct_current_model")]
    pub _current_model: Option<String>,
    #[serde(default, rename = "__ct_parent_workspace")]
    pub parent_workspace: Option<PathBuf>,
    #[serde(default, rename = "__ct_session_id")]
    pub parent_session_id: Option<String>,
    #[serde(default, rename = "__ct_prior_context_allowed")]
    pub parent_prior_context_allowed: Option<bool>,
}

impl Params {
    /// Returns the effective detach flag, defaulting to synchronous execution.
    pub(super) fn detach_or_default(&self) -> bool {
        self.detach.unwrap_or(false)
    }

    pub(super) fn detach_for_spawn(&self) -> bool {
        self.detach.unwrap_or(false)
    }

    pub(super) fn resume_config(&self) -> super::residency::ResumeConfig {
        super::residency::ResumeConfig::new(
            self._current_model.clone(),
            self.parent_workspace.clone(),
            self.parent_prior_context_allowed,
        )
    }
}
