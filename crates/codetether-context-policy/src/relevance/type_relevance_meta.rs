//! Shared RelevanceMeta contract.
use super::*;

/// Per-entry syntactic relevance signals.
///
/// Parallel-array friendly: one [`RelevanceMeta`] per entry in
/// `Session::messages` in the application. Lives in a
/// `<session-id>.relevance.jsonl` sidecar once wired into
/// `Session::save` (a future commit).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelevanceMeta {
    /// Path-like tokens found in the message's text parts.
    #[serde(default)]
    pub files: Vec<String>,
    /// Names of any `ToolCall` or `ToolResult` content parts.
    #[serde(default)]
    pub tools: Vec<String>,
    /// Short error-class tags surfaced by common error markers
    /// (`Error:`, `error[E`, `failed`, `panicked`, `Traceback`).
    #[serde(default)]
    pub error_classes: Vec<String>,
    /// Message indices this entry explicitly references
    /// (reserved for future N-back extraction; empty in Phase B v1).
    #[serde(default)]
    pub explicit_refs: Vec<usize>,
}
