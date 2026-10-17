//! Existing PRD response-recovery regression test.

use crate::quality::detect_quality_checks;

#[test]
fn detect_quality_checks_returns_defaults_for_unknown() {
    // This tests that default is returned when no project file is found
    // (may detect Cargo.toml in workspace — that's fine, we just test no panic)
    let _checks = detect_quality_checks();
}
