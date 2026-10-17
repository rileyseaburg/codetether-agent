//! encoding wire regression.
use crate::{ChatEvent, Event};

#[test]
fn wire_encoding_and_round_trip() {
    let event = ChatEvent::ToolResult {
        tool_name: "read".into(),
        result: "ok".into(),
        success: true,
        timestamp: 42,
    };
    assert_eq!(
        event.to_json(),
        r#"{"type":"tool_result","tool_name":"read","result":"ok","success":true,"timestamp":42}"#
    );
    let decoded: ChatEvent = serde_json::from_str(&event.to_json()).unwrap();
    assert!(matches!(
        decoded,
        ChatEvent::ToolResult {
            success: true,
            timestamp: 42,
            ..
        }
    ));
    assert_eq!(
        serde_json::to_value(Event::Chat(event)).unwrap()["type"],
        "tool_result"
    );
}
