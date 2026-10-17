//! Typed context failure reasons and their serialized/display representation.
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Typed fault reason codes surfaced from context construction,
/// recall, and writeback.
///
/// Each variant is a terminal outcome that should be logged or
/// rendered in the audit view; none of them is an internal retry
/// signal. Callers that want to distinguish recoverable errors from
/// terminal faults should match on the variant.
///
/// # Examples
/// ```rust
/// use codetether_context_policy::fault::Fault;
/// let fault=Fault::Denied{reason:"policy rejected access".into()};
/// assert!(fault.is_denied());
/// assert_eq!(fault.code(),"denied");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Fault {
    /// The query ran to completion but produced no matching entries.
    /// Distinct from an error — this is the empty-result reason code.
    #[error("no match")]
    NoMatch,
    /// The backend rejected the request (policy, auth, rate-limit).
    #[error("denied: {reason}")]
    Denied { reason: String },
    /// The backend returned a transport / IO error.
    #[error("backend error: {reason}")]
    BackendError { reason: String },
    /// A dirty page was dropped because the writeback protocol was
    /// skipped or bypassed.
    #[error("flush miss")]
    FlushMiss,
    /// A `Bootstrap` page that should have been pinned was missing
    /// after compaction / reset.
    #[error("post-compaction bootstrap fault")]
    PostCompactionBootstrap,
    /// A hard-pinned page was not present at prompt-assembly time.
    #[error("pinned invariant miss")]
    PinnedInvariantMiss,
    /// An equivalent tool call ran again because its previous result
    /// was evicted.
    #[error("duplicate tool")]
    DuplicateTool,
    /// A tool result had to be re-fetched after eviction.
    #[error("refetch")]
    Refetch,
}
