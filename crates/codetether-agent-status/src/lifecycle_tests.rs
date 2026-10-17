//! Regression tests for sub-agent lifecycle reporting.

use super::{result_message, terminal_summary};

#[test]
fn failed_summary_preserves_error_before_first_tool_call() {
    let summary = terminal_summary(0, Some("provider initialization failed"));
    assert_eq!(
        summary,
        "0 tool call(s); error: provider initialization failed"
    );
}

#[test]
fn successful_summary_reports_tool_count() {
    assert_eq!(terminal_summary(2, None), "2 tool call(s)");
}

#[test]
fn completed_result_contains_the_actual_response() {
    let message = result_message("reviewer", "found the regression", None);
    assert!(message.contains("@reviewer completed"));
    assert!(message.contains("found the regression"));
}
