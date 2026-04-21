//! Observable faults with typed reason codes (ClawVM §3).
//!
//! Core invariant: no subsystem that participates in context
//! construction or recall ever returns an *empty* result silently. A
//! genuine "no match" is a [`Fault::NoMatch`], a denied backend is a
//! [`Fault::Denied`], and so on. This makes every recall / derivation
//! failure diagnosable without adding instrumentation after the fact.
//!
//! ClawVM shows that silent returns (their "silent-recall" failure
//! family) are one of the three root causes of agent memory bugs in
//! the field. Surfacing them as typed reason codes is the Phase A
//! structural remedy.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::session::faults::Fault;
//!
//! // Distinct from `Option::None` — a caller can tell an empty
//! // archive apart from an un-authorised backend from a transport
//! // error.
//! let f = Fault::Denied {
//!     reason: "policy rejected session_recall".into(),
//! };
//! assert!(f.is_denied());
//! ```

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Typed fault reason codes surfaced from context construction,
/// recall, and writeback.
///
/// Each variant is a terminal outcome that should be logged or
/// rendered in the audit view; none of them is an internal retry
/// signal. Callers that want to distinguish recoverable errors from
/// terminal faults should match on the variant.
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

impl Fault {
    /// `true` iff the fault is the empty-result reason code.
    pub fn is_no_match(&self) -> bool {
        matches!(self, Fault::NoMatch)
    }

    /// `true` iff the fault is a policy / auth rejection.
    pub fn is_denied(&self) -> bool {
        matches!(self, Fault::Denied { .. })
    }

    /// Short machine-readable reason-code string suitable for logs
    /// and TUI audit view filtering.
    pub fn code(&self) -> &'static str {
        match self {
            Fault::NoMatch => "no_match",
            Fault::Denied { .. } => "denied",
            Fault::BackendError { .. } => "backend_error",
            Fault::FlushMiss => "flush_miss",
            Fault::PostCompactionBootstrap => "post_compaction_bootstrap",
            Fault::PinnedInvariantMiss => "pinned_invariant_miss",
            Fault::DuplicateTool => "duplicate_tool",
            Fault::Refetch => "refetch",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_match_is_its_own_thing() {
        let f = Fault::NoMatch;
        assert!(f.is_no_match());
        assert!(!f.is_denied());
        assert_eq!(f.code(), "no_match");
        assert_eq!(f.to_string(), "no match");
    }

    #[test]
    fn denied_carries_reason_string() {
        let f = Fault::Denied {
            reason: "policy rejected".into(),
        };
        assert!(f.is_denied());
        assert!(f.to_string().contains("policy rejected"));
        assert_eq!(f.code(), "denied");
    }

    #[test]
    fn all_codes_are_unique_snake_case() {
        let faults = [
            Fault::NoMatch,
            Fault::Denied { reason: "x".into() },
            Fault::BackendError { reason: "y".into() },
            Fault::FlushMiss,
            Fault::PostCompactionBootstrap,
            Fault::PinnedInvariantMiss,
            Fault::DuplicateTool,
            Fault::Refetch,
        ];
        let codes: Vec<&'static str> = faults.iter().map(Fault::code).collect();
        let mut seen = codes.clone();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), codes.len(), "reason codes must be unique");
        for c in &codes {
            assert!(!c.is_empty());
            assert_eq!(c.to_ascii_lowercase(), *c, "codes must be snake_case");
        }
    }

    #[test]
    fn fault_round_trips_through_serde() {
        let f = Fault::Denied {
            reason: "policy".into(),
        };
        let json = serde_json::to_string(&f).unwrap();
        assert!(json.contains("\"kind\":\"denied\""));
        let back: Fault = serde_json::from_str(&json).unwrap();
        assert_eq!(back, f);
    }
}
