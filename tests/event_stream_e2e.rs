//! End-to-end test for event streaming

#[cfg(test)]
mod tests {
    use codetether_agent::event_stream::{ChatEvent, EventFile};
    use std::path::PathBuf;

    #[test]
    fn test_event_stream_integration() {
        // Test that event streaming infrastructure is properly wired

        // 1. Test ChatEvent can be created with tool result data
        let event = ChatEvent::tool_result(
            PathBuf::from("/test/workspace"),
            "test-session-123".to_string(),
            "bash",
            true,
            22515,
            "✓ bash output",
            1,
        );

        let json = event.to_json();
        let parsed: ChatEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.session_id, "test-session-123");
        assert_eq!(parsed.tool_name, Some("bash".to_string()));
        assert_eq!(parsed.tool_success, Some(true));
        assert_eq!(parsed.tool_duration_ms, Some(22515));

        // 2. Test filename format with byte-range offsets
        let filename = EventFile::filename("test-session-123", 1000, 2500);
        assert!(filename.contains("chat-events-"));
        assert!(filename.ends_with(".jsonl"));

        println!("✓ Event stream integration test passed");
        println!("  - Event JSON: {}", json);
        println!("  - Filename: {}", filename);
    }
}
