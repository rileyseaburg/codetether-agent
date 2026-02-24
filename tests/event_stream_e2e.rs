//! End-to-end test for event streaming

#[cfg(test)]
mod tests {
    use codetether_agent::event_stream::ChatEvent;
    use std::path::PathBuf;

    #[test]
    fn test_event_stream_integration() {
        // 1. ChatEvent tool result serialization/deserialization
        let event = ChatEvent::tool_result(
            PathBuf::from("/test/workspace"),
            "test-session-123".to_string(),
            "bash",
            true,
            22_515,
            "ok",
            1,
        );

        let json = event.to_json();
        let parsed: ChatEvent = serde_json::from_str(&json).expect("chat event should deserialize");

        match parsed {
            ChatEvent::ToolResult {
                tool_name,
                result,
                success,
                timestamp,
            } => {
                assert_eq!(tool_name, "bash");
                assert_eq!(result, "ok");
                assert!(success);
                assert!(timestamp > 0);
            }
            other => panic!("unexpected event variant: {other:?}"),
        }

        // 2. Filename format used for event-stream byte-range files
        // Format: {timestamp}-chat-events-{start_offset}-{end_offset}.jsonl
        let timestamp = chrono::Utc::now().format("%Y%m%dT%H%M%SZ");
        let filename = format!(
            "{}-chat-events-{:020}-{:020}.jsonl",
            timestamp, 1000u64, 2500u64
        );

        assert!(filename.contains("chat-events-"));
        assert!(filename.ends_with(".jsonl"));

        // Ensure start/end offsets can be recovered from filename with current parser strategy.
        let parts: Vec<&str> = filename.split('-').collect();
        let start: u64 = parts[parts.len() - 2]
            .parse()
            .expect("start offset should parse");
        let end: u64 = parts[parts.len() - 1]
            .trim_end_matches(".jsonl")
            .parse()
            .expect("end offset should parse");

        assert_eq!(start, 1000);
        assert_eq!(end, 2500);
    }
}
