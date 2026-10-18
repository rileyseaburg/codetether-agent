//! Legacy TUI convergence regression.
use crate::normalize_for_convergence;

#[test]
fn normalize_for_convergence_ignores_case_and_punctuation() {
    let a = normalize_for_convergence("Done! Next Step: Add tests.", 280);
    let b = normalize_for_convergence("done next step add tests", 280);
    assert_eq!(a, b);
}
