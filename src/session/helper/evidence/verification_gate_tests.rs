use super::apply;
use crate::session::helper::evidence::gate_mode::GateMode;

#[test]
fn warns_on_unlabeled_no_change_claim() {
    let answer = apply(GateMode::Warn, "No files were changed.");
    assert!(answer.contains("Evidence gate warning"));
}

#[test]
fn strict_mode_withholds_unsupported_completion_claim() {
    let answer = apply(GateMode::Strict, "Implementation complete.");
    assert!(answer.contains("verification claim withheld"));
    assert!(!answer.contains("Implementation complete"));
}

#[test]
fn accepts_labeled_claim_with_command_source() {
    let claim = "Focused CI-like: tests passed via `cargo test unit_name`.";
    assert_eq!(apply(GateMode::Strict, claim), claim);
}

#[test]
fn accepts_explicit_not_run_claim() {
    let claim = "Not-run: no tests were run.";
    assert_eq!(apply(GateMode::Strict, claim), claim);
}

#[test]
fn label_without_source_is_rejected() {
    let answer = apply(GateMode::Strict, "Static/local: tests passed.");
    assert!(answer.contains("concrete evidence source"));
}
