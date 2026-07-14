//! Core data types for [`Session`]: the session struct itself, its
//! persistent metadata, and the image attachment helper used by multimodal
//! prompts.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::agent::ToolUse;
use crate::provider::{Message, Usage};
use crate::session::index::SummaryIndex;
use crate::session::pages::PageKind;

#[path = "metadata.rs"]
mod metadata;
pub use metadata::SessionMetadata;

/// Default maximum agentic loop iterations when [`Session::max_steps`] is
/// `None`. 250 was far too generous — most tasks converge in under 20 steps
/// and anything past 50 usually means the model is spinning without progress.
pub const DEFAULT_MAX_STEPS: usize = 50;

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
    /// Durable session configuration.
    ///
    /// Serialized *before* [`Self::messages`] so cheap workspace-match
    /// prefiltering (see [`crate::session::header::SessionHeader`]) can
    /// avoid lexing past the transcript.
    pub metadata: SessionMetadata,
    /// Name of the agent persona that owns this session.
    pub agent: String,
    /// Ordered conversation transcript.
    #[serde(deserialize_with = "crate::session::tail_seed::deserialize_tail_vec")]
    pub messages: Vec<Message>,
    /// Per-message page classification sidecar.
    ///
    /// Backfilled on load for legacy sessions that predate the
    /// history/context split.
    #[serde(default)]
    pub pages: Vec<PageKind>,
    /// Hierarchical summary cache used as the seed for proactive RLM
    /// preparation and invalidated on the [`Session::add_message`] hot path.
    #[serde(default)]
    pub summary_index: SummaryIndex,
    /// Per-tool-call audit records.
    #[serde(deserialize_with = "crate::session::tail_seed::deserialize_tail_vec")]
    pub tool_uses: Vec<ToolUse>,
    /// Aggregate token usage across all completions in this session.
    pub usage: Usage,
    /// Maximum agentic loop steps. [`None`] falls back to
    /// [`DEFAULT_MAX_STEPS`].
    #[serde(skip)]
    pub max_steps: Option<usize>,
    /// Optional bus for publishing agent thinking/reasoning.
    #[serde(skip)]
    pub bus: Option<Arc<crate::bus::AgentBus>>,
}
