//! Shared fixtures for Z.AI stream-termination tests.

use crate::provider::zai::stream_output::OutputSeen;
use crate::provider::zai::zai_stream_state::ZaiStreamToolState;
use std::collections::HashMap;

/// A tool call that has started streaming but has not been closed.
pub(super) fn open_tool_state() -> HashMap<usize, ZaiStreamToolState> {
    let mut states = HashMap::new();
    states.insert(
        0,
        ZaiStreamToolState {
            stream_id: "call_1".to_string(),
            name: Some("bash".to_string()),
            started: true,
            finished: false,
        },
    );
    states
}

/// Output state for a turn whose only output was a tool call.
pub(super) fn tool_only() -> OutputSeen {
    OutputSeen {
        text: false,
        tool_calls: true,
    }
}
