//! Decide how an isolated branch may be integrated.

/// Integration decision: merge, cleanup, or reject missing/unexpected edits.
/// # Examples
/// ```
/// use codetether_worktree::swarm::branch::policy::{decide, Decision};
/// assert_eq!(decide(true, true), Decision::Merge);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Merge the worker patch.
    Merge,
    /// Clean up a verification worktree with no patch.
    Cleanup,
    /// Reject a mutating worker without a patch.
    RejectMissingChanges,
    /// Retain unexpected edits from a non-mutating worker.
    RejectUnexpectedChanges,
}

/// Select branch integration behavior from expected and observed changes.
/// # Arguments
/// * `expects_changes` — Whether a patch is required.
/// * `net_changes` — Whether a patch exists.
/// # Returns
/// The merge, cleanup or rejection decision.
/// # Examples
/// ```no_run
/// use codetether_worktree::swarm::branch::policy::{decide, Decision};
/// assert_eq!(decide(false, true), Decision::RejectUnexpectedChanges);
/// ```
pub fn decide(expects_changes: bool, net_changes: bool) -> Decision {
    match (expects_changes, net_changes) {
        (true, true) => Decision::Merge,
        (true, false) => Decision::RejectMissingChanges,
        (false, true) => Decision::RejectUnexpectedChanges,
        (false, false) => Decision::Cleanup,
    }
}

#[cfg(test)]
mod tests {
    use super::{Decision, decide};

    #[test]
    fn verification_never_merges_source_changes() {
        assert_eq!(decide(false, false), Decision::Cleanup);
        assert_eq!(decide(false, true), Decision::RejectUnexpectedChanges);
    }

    #[test]
    fn mutation_requires_a_net_patch() {
        assert_eq!(decide(true, false), Decision::RejectMissingChanges);
        assert_eq!(decide(true, true), Decision::Merge);
    }
}
