//! Core data types for [`Session`]: the session struct itself, its
//! persistent metadata, and the image attachment helper used by multimodal
//! prompts.

use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::agent::ToolUse;
use crate::provenance::ExecutionProvenance;
use crate::provider::{Message, Usage};

/// Default maximum agentic loop iterations when [`Session::max_steps`] is
/// `None`.
pub const DEFAULT_MAX_STEPS: usize = 250;

/// An image attachment to include with a user message (e.g. pasted from the
/// clipboard in the TUI).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::ImageAttachment;
///
/// let img = ImageAttachment {
///     data_url: "data:image/png;base64,iVBORw0KGgo...".to_string(),
///     mime_type: Some("image/png".to_string()),
/// };
/// assert!(img.data_url.starts_with("data:image/png;base64"));
/// assert_eq!(img.mime_type.as_deref(), Some("image/png"));
/// ```
#[derive(Debug, Clone)]
pub struct ImageAttachment {
    /// Base64-encoded data URL, e.g. `"data:image/png;base64,iVBOR..."`.
    pub data_url: String,
    /// MIME type of the image, e.g. `"image/png"`.
    pub mime_type: Option<String>,
}

/// A conversation session.
///
/// See the [`session`](crate::session) module docs for a usage overview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// UUID identifying this session on disk.
    pub id: String,
    /// Optional human-readable title. Auto-generated from the first user
    /// message when absent; see [`Session::generate_title`](crate::session::Session::generate_title).
    pub title: Option<String>,
    /// When the session was first created.
    pub created_at: DateTime<Utc>,
    /// When the session was last modified.
    pub updated_at: DateTime<Utc>,
    /// Ordered conversation transcript.
    pub messages: Vec<Message>,
    /// Per-tool-call audit records.
    pub tool_uses: Vec<ToolUse>,
    /// Aggregate token usage across all completions in this session.
    pub usage: Usage,
    /// Name of the agent persona that owns this session.
    pub agent: String,
    /// Durable session configuration.
    pub metadata: SessionMetadata,
    /// Maximum agentic loop steps. [`None`] falls back to
    /// [`DEFAULT_MAX_STEPS`].
    #[serde(skip)]
    pub max_steps: Option<usize>,
    /// Optional bus for publishing agent thinking/reasoning.
    #[serde(skip)]
    pub bus: Option<Arc<crate::bus::AgentBus>>,
}

/// Persistent, user-facing configuration for a [`Session`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// Workspace directory the session operates in.
    pub directory: Option<PathBuf>,
    /// Model selector in `"provider/model"` or `"model"` form.
    pub model: Option<String>,
    /// Optional snapshot of the workspace knowledge graph.
    pub knowledge_snapshot: Option<PathBuf>,
    /// Execution provenance for audit/traceability.
    pub provenance: Option<ExecutionProvenance>,
    /// When true, pending edit previews are auto-confirmed in the TUI.
    #[serde(default)]
    pub auto_apply_edits: bool,
    /// When true, network-reaching tools are allowed.
    #[serde(default)]
    pub allow_network: bool,
    /// When true, the TUI shows slash-command autocomplete.
    #[serde(default = "default_slash_autocomplete")]
    pub slash_autocomplete: bool,
    /// When true, the CLI runs inside an isolated git worktree.
    #[serde(default = "default_use_worktree")]
    pub use_worktree: bool,
    /// Whether this session has been shared publicly.
    pub shared: bool,
    /// Public share URL, if any.
    pub share_url: Option<String>,
}

fn default_slash_autocomplete() -> bool {
    true
}

fn default_use_worktree() -> bool {
    true
}
