//! Deserializable agent-tool wire payload.
use codetether_collaboration_input::MessageImage;
use serde::Deserialize;
use std::path::PathBuf;
/// Agent-tool payload retaining runtime-injected fields and optional defaults.
/// # Examples
/// ```rust
/// let params: codetether_agent_input::Params = serde_json::from_str(r#"{"action":"list"}"#).unwrap();
/// assert!(!params.detach_or_default());
/// ```
#[derive(Deserialize)]
pub struct Params {
    /// Requested agent-tool operation.
    pub action: String,
    #[serde(default)]
    /// Agent display name or target identity.
    pub name: Option<String>,
    #[serde(default)]
    /// Instructions used when spawning a child.
    pub instructions: Option<String>,
    #[serde(default)]
    /// Message text for a child or remote agent.
    pub message: Option<String>,
    /// Reusable A2A conversation identifier for a remote message.
    #[serde(default)]
    pub context_id: Option<String>,
    #[serde(default, rename = "__ct_message_images")]
    /// Attached images injected by the tool runtime.
    pub message_images: Vec<MessageImage>,
    #[serde(default)]
    /// Explicit provider/model override.
    pub model: Option<String>,
    #[serde(default)]
    /// Whether the child is ephemeral.
    pub ephemeral: bool,
    /// Controls whether `message`/`spawn` blocks the caller.
    ///
    /// `None` (default) waits for the child result so model callers cannot
    /// finish before their delegated work. Interactive callers opt into detach.
    #[serde(default)]
    pub detach: Option<bool>,
    #[serde(default)]
    /// Requested parent-history inheritance scope.
    pub fork_turns: Option<String>,
    #[serde(default, rename = "__ct_current_model")]
    /// Current parent model injected by the runtime.
    pub _current_model: Option<String>,
    #[serde(default, rename = "__ct_parent_workspace")]
    /// Parent workspace injected by the runtime.
    pub parent_workspace: Option<PathBuf>,
    #[serde(default, rename = "__ct_session_id")]
    /// Parent durable identity injected by the runtime.
    pub parent_session_id: Option<String>,
    #[serde(default, rename = "__ct_prior_context_allowed")]
    /// Inherited prior-context access ceiling.
    pub parent_prior_context_allowed: Option<bool>,
}
