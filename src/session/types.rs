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
use crate::session::delegation::DelegationState;
use crate::session::derive_policy::DerivePolicy;
use crate::session::history_sink::HistorySinkConfig;
use crate::session::index::SummaryIndex;
use crate::session::pages::PageKind;

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
    /// Hierarchical summary cache — Phase B step 14. Populated lazily
    /// by step 18's producer and invalidated on the
    /// [`Session::add_message`] hot path.
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

/// Persistent, user-facing configuration for a [`Session`].
#[derive(Clone, Default, Serialize, Deserialize)]
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
    /// RLM (Recursive Language Model) settings active for this session.
    ///
    /// Seeded from [`crate::config::Config::rlm`] at session creation
    /// (via [`crate::session::Session::apply_config`]) and persisted on
    /// disk so subsequent runs honour the same thresholds, models, and
    /// iteration limits. Existing sessions without this field load with
    /// [`crate::rlm::RlmConfig::default`].
    #[serde(default)]
    pub rlm: crate::rlm::RlmConfig,
    /// Per-session context derivation policy.
    ///
    /// Defaults to [`DerivePolicy::Legacy`]. The prompt loop can still
    /// override this at runtime via `CODETETHER_CONTEXT_POLICY`.
    #[serde(default)]
    pub context_policy: DerivePolicy,
    /// CADMAS-CTX routing posteriors for this session.
    #[serde(default)]
    pub delegation: DelegationState,
    /// Runtime MinIO / S3 history sink configuration.
    ///
    /// Intentionally not serialized: carrying live access keys inside the
    /// session JSON would persist secrets to disk.
    #[serde(skip)]
    pub history_sink: Option<HistorySinkConfig>,
    /// Pre-resolved subcall provider from
    /// [`RlmConfig::subcall_model`](crate::rlm::RlmConfig::subcall_model).
    ///
    /// Not serialised — re-resolved from the provider registry each time
    /// [`Session::apply_config`] runs. When `None`, all RLM iterations
    /// use the root provider.
    #[serde(skip)]
    pub subcall_provider: Option<std::sync::Arc<dyn crate::provider::Provider>>,
    /// Model name resolved alongside [`Self::subcall_provider`].
    /// Not serialised.
    #[serde(skip)]
    pub subcall_model_name: Option<String>,
}

impl std::fmt::Debug for SessionMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionMetadata")
            .field("directory", &self.directory)
            .field("model", &self.model)
            .field("knowledge_snapshot", &self.knowledge_snapshot)
            .field("provenance", &self.provenance)
            .field("auto_apply_edits", &self.auto_apply_edits)
            .field("allow_network", &self.allow_network)
            .field("slash_autocomplete", &self.slash_autocomplete)
            .field("use_worktree", &self.use_worktree)
            .field("shared", &self.shared)
            .field("share_url", &self.share_url)
            .field("rlm", &self.rlm)
            .field("context_policy", &self.context_policy)
            .field("delegation", &self.delegation)
            .field(
                "history_sink",
                &self
                    .history_sink
                    .as_ref()
                    .map(|cfg| (&cfg.endpoint, &cfg.bucket, &cfg.prefix)),
            )
            .field(
                "subcall_provider",
                &self.subcall_provider.as_ref().map(|_| "<provider>"),
            )
            .field("subcall_model_name", &self.subcall_model_name)
            .finish()
    }
}

fn default_slash_autocomplete() -> bool {
    true
}

fn default_use_worktree() -> bool {
    true
}
