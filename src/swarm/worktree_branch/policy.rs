//! Decide how an isolated branch may be integrated.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Decision {
    Merge,
    Cleanup,
    RejectMissingChanges,
    RejectUnexpectedChanges,
}

pub(crate) fn decide(expects_changes: bool, net_changes: bool) -> Decision {
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
