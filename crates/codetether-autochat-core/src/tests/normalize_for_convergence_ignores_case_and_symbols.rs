//! Existing auto-chat parsing and policy regression.
use super::*;

#[test]
fn normalize_for_convergence_ignores_case_and_symbols() {
    let a = normalize_for_convergence("Done! Next Step: Add tests.", 280);
    let b = normalize_for_convergence("done next step add tests", 280);
    assert_eq!(a, b);
}
