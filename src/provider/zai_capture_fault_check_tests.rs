//! Tests for the empty-stream fault decision, which combines observed output
//! with `finish_reason`.

use super::check;
use crate::provider::zai::stream_output::OutputSeen;

fn nothing() -> OutputSeen {
    OutputSeen::default()
}

#[test]
fn suppresses_error_when_model_stopped() {
    assert!(check(nothing(), 3, Some("stop")).is_none());
}

#[test]
fn suppresses_error_when_text_was_produced() {
    let text_only = OutputSeen {
        text: true,
        tool_calls: false,
    };
    assert!(check(text_only, 3, None).is_none());
}

#[test]
fn tool_only_turn_without_finish_reason_is_not_a_fault() {
    // Live regression: a turn whose only output was a bash tool call ended
    // with no finish_reason after 99 frames. Faulting restarted the completed
    // turn five times and then failed the turn.
    let tool_only = OutputSeen {
        text: false,
        tool_calls: true,
    };
    assert!(check(tool_only, 99, None).is_none());
}

#[test]
fn reports_truncated_stream() {
    assert!(check(nothing(), 0, None).is_some());
}
