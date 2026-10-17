//! long history preserves sinks and recency.
use super::*;

#[test]
pub(super) fn long_history_preserves_sinks_and_recency() {
    let mut msgs = vec![u("SINK0"), a("SINK1")];
    for i in 0..60 {
        msgs.push(u(&format!("mid-user-{i}")));
        msgs.push(a(&"X".repeat(300)));
    }
    for i in 0..RECENT_MESSAGES {
        msgs.push(a(&format!("recent-{i}")));
    }
    let stats = trim_middle(&mut msgs);
    assert!(stats.total_bytes_saved > 1000);
    // Sink intact.
    let ContentPart::Text { text } = &msgs[0].content[0] else {
        panic!();
    };
    assert_eq!(text, "SINK0");
    // A streaming-llm marker appears somewhere after the sinks.
    assert!(msgs.iter().any(|m| m.content.iter().any(|p| matches!(
        p,
        ContentPart::Text { text } if text.contains("streaming-llm trim")
    ))));
}
