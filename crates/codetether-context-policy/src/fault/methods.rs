//! Stable fault predicates and machine-readable reason codes.
use super::Fault;

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
