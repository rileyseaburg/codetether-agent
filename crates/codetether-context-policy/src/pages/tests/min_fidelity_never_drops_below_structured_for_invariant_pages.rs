//! Existing page policy regression.
use super::*;

#[test]
fn min_fidelity_never_drops_below_structured_for_invariant_pages() {
    assert_eq!(
        PageKind::Bootstrap.min_fidelity(),
        ResidencyLevel::Structured
    );
    assert_eq!(
        PageKind::Constraint.min_fidelity(),
        ResidencyLevel::Structured
    );
}
