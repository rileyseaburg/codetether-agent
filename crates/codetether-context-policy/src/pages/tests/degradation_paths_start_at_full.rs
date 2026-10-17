//! Existing page policy regression.
use super::*;

#[test]
fn degradation_paths_start_at_full() {
    for kind in [
        PageKind::Bootstrap,
        PageKind::Constraint,
        PageKind::Plan,
        PageKind::Preference,
        PageKind::Evidence,
        PageKind::Conversation,
    ] {
        assert_eq!(kind.degradation_path()[0], ResidencyLevel::Full);
    }
}
