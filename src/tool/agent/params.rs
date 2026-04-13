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
    #[serde(default, rename = "__ct_current_model")]
    pub _current_model: Option<String>,
}
