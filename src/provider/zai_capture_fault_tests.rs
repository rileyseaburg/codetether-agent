//! Tests for `finish_reason` classification and diagnostic message rendering.

use super::{is_fault, message};

#[test]
fn stated_stop_is_not_a_fault() {
    // GLM ends a turn after a tool result with no new content and
    // finish_reason "stop". Faulting here restarted a completed stream.
    assert!(!is_fault(Some("stop")));
}

#[test]
fn tool_calls_finish_is_not_a_fault() {
    assert!(!is_fault(Some("tool_calls")));
}

#[test]
fn missing_finish_reason_is_a_fault() {
    assert!(is_fault(None));
}

#[test]
fn abnormal_finish_reasons_remain_faults() {
    assert!(is_fault(Some("length")));
    assert!(is_fault(Some("content_filter")));
}

#[test]
fn message_reports_diagnostics() {
    let text = message(0, None);

    assert!(text.contains("frames=0"));
    assert!(text.contains("finish_reason=none"));
    assert!(text.contains("CODETETHER_ZAI_CAPTURE_DIR"));
}

#[test]
fn message_includes_finish_reason() {
    assert!(message(7, Some("tool_calls")).contains("finish_reason=tool_calls"));
}
