//! Regressions for deliverable metadata versus actual failures.

use super::error;

#[test]
fn useful_output_does_not_require_a_status_footer() {
    for output in [
        "Evidence: inspected the requested diff and identified two defects.",
        "Review findings:\n- Routing telemetry retains an alias.\n",
        "evidence\nSTATUS: completed",
        "evidence. STATUS: completed",
        "evidence\n**STATUS: COMPLETED**\n\n",
    ] {
        assert_eq!(error(output), None, "output: {output:?}");
    }
}

#[test]
fn explicit_non_completion_is_still_a_failure() {
    for output in [
        "evidence\nSTATUS: blocked",
        "next action\nSTATUS: pending",
        "evidence\n**STATUS: BLOCKED**\n\n",
        "next action\n`STATUS: pending`",
        "blocked",
        "pending",
    ] {
        assert!(error(output).is_some(), "output: {output:?}");
    }
}

#[test]
fn empty_output_and_invalid_explicit_status_are_failures() {
    for output in ["", "\n \t\n", "evidence\nSTATUS: unknown", "STATUS:"] {
        assert!(error(output).is_some(), "output: {output:?}");
    }
}

#[test]
fn missing_footer_is_not_reported_as_a_model_or_runtime_failure() {
    assert_eq!(
        error("Static/local review: no implementation or executed-test proof."),
        None
    );
}
