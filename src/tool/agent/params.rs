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
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub ephemeral: bool,
    /// Controls whether `message`/`spawn` blocks the caller.
    ///
    /// `None` (default) → non-blocking (detach = true): the sub-agent runs in
    /// the background and the caller is free to continue (issue #296).
    /// `Some(false)` → blocking: waits for the full sub-agent turn.
    #[serde(default)]
    pub detach: Option<bool>,
    #[serde(default, rename = "__ct_current_model")]
    pub _current_model: Option<String>,
    #[serde(default, rename = "__ct_parent_workspace")]
    pub parent_workspace: Option<PathBuf>,
}

impl Params {
    /// Returns the effective detach flag, defaulting to `true` (non-blocking)
    /// when not explicitly set (issue #296).
    pub(super) fn detach_or_default(&self) -> bool {
        self.detach.unwrap_or(true)
    }
}
