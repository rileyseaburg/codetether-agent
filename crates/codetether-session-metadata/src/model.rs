//! Persistent configuration attached to one conversation session.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::defaults as support;

/// Persistent, user-facing configuration for a session.
/// # Examples
/// ```rust
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// metadata.model = Some("provider/model".into());
/// assert_eq!(metadata.model.as_deref(), Some("provider/model"));
/// ```
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// Workspace directory the session operates in.
    pub directory: Option<PathBuf>,
    /// Keep a task-owned checkout when parent settings are refreshed on resume.
    /// Spawned worktree children pin their directory for their entire lifetime.
    #[serde(default)]
    pub workspace_pinned: bool,
    /// Provider/model selector for the session.
    pub model: Option<String>,
    /// Optional snapshot of the workspace knowledge graph.
    pub knowledge_snapshot: Option<PathBuf>,
    /// Execution provenance retained for audit records.
    pub provenance: Option<codetether_provenance::ExecutionProvenance>,
    /// Whether pending edit previews are applied automatically.
    #[serde(default)]
    pub auto_apply_edits: bool,
    /// Whether network-reaching tools may run.
    #[serde(default)]
    pub allow_network: bool,
    /// Durable user choice for prior memory/session/history access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_context_allowed: Option<bool>,
    /// Human-authorized override for the current turn only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_context_turn_allowed: Option<bool>,
    /// Hard access ceiling inherited from an autonomous parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited_prior_context_allowed: Option<bool>,
    /// Whether slash-command autocomplete is enabled.
    #[serde(default = "support::slash_autocomplete")]
    pub slash_autocomplete: bool,
    /// Whether commands should use an isolated worktree.
    #[serde(default = "support::use_worktree")]
    pub use_worktree: bool,
    /// Whether the session has been publicly shared.
    pub shared: bool,
    /// Public share URL when sharing is active.
    pub share_url: Option<String>,
    /// Recursive language-model settings.
    #[serde(default)]
    pub rlm: codetether_rlm::RlmConfig,
    /// Per-session context derivation policy.
    #[serde(default)]
    pub context_policy: codetether_context_policy::derivation::DerivePolicy,
    /// Learned delegation routing state.
    #[serde(default)]
    pub delegation: codetether_context_policy::delegation::DelegationState,
    /// Resumable agent-loop checkpoint.
    #[serde(default)]
    pub run_checkpoint: Option<codetether_run_checkpoint::RunCheckpoint>,
    /// Durable tombstone preventing an old fallback sidecar from reviving a cleared run.
    #[serde(default)]
    pub run_checkpoint_cleared: bool,
    /// Bounded source-linked tool observations shared by all session continuations.
    #[serde(default)]
    pub progress: codetether_repeat_guard::ProgressLedger,
    /// Exact source spans replaced by later user messages, retained across eviction.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraint_corrections: Vec<codetether_context_policy::state_header::ConstraintCorrection>,
    /// Runtime-only object-storage history sink configuration.
    #[serde(skip)]
    pub history_sink: Option<codetether_history_sink::HistorySinkConfig>,
    /// Runtime-only provider credentials; never serialized.
    #[serde(skip)]
    pub provider_keys: Option<serde_json::Value>,
    /// Runtime-only provider selected for RLM subcalls.
    #[serde(skip)]
    pub subcall_provider: Option<std::sync::Arc<dyn codetether_provider_core::Provider>>,
    /// Runtime-only model name paired with the subcall provider.
    #[serde(skip)]
    pub subcall_model_name: Option<String>,
}
