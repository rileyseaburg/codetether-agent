//! encoding constructor regression.
use crate::{ChatEvent, Event};

#[test]
fn tool_result_constructor() {
    let before = chrono::Utc::now().timestamp();
    let result = ChatEvent::tool_result(
        "/unused".into(),
        "session".into(),
        "write",
        false,
        17,
        "denied",
        4,
    );
    match result {
        ChatEvent::ToolResult {
            tool_name,
            result,
            success,
            timestamp,
        } => {
            assert_eq!(tool_name, "write");
            assert_eq!(result, "denied");
            assert!(!success);
            assert!(timestamp >= before);
            assert!(timestamp <= chrono::Utc::now().timestamp());
        }
        _ => panic!("unexpected event"),
    }
}
