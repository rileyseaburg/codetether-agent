use crate::bus::BusMessage;

use super::tool_calls::ToolCallTracker;
use super::tool_calls::observe::MAX_OPEN_CALLS;

#[test]
fn missing_responses_cannot_grow_the_tracker_forever() {
    let mut tracker = ToolCallTracker::default();
    for index in 0..MAX_OPEN_CALLS + 50 {
        tracker.observe(&request(index));
    }
    assert_eq!(tracker.open_calls().count(), MAX_OPEN_CALLS);
}

fn request(index: usize) -> BusMessage {
    BusMessage::ToolRequest {
        request_id: format!("request-{index}"),
        agent_id: "agent".into(),
        tool_name: "tool".into(),
        arguments: serde_json::Value::Null,
        step: index,
    }
}
