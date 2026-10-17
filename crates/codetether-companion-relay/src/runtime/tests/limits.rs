//! Queue validation and acknowledgement contracts.
use super::fixture::runtime;
use serde_json::json;

#[test]
fn reply_limits_count_utf16_and_preserve_text() {
    let mut state = runtime();
    for text in ["".to_owned(), " \n ".to_owned(), "😀".repeat(1001)] {
        assert_eq!(
            state
                .queue_reply(&json!({"text": text}), true, 100)
                .err()
                .unwrap()
                .status,
            400
        );
    }
    let text = "😀".repeat(1000);
    let receipt = state
        .queue_reply(&json!({"text": text}), true, 100)
        .unwrap();
    assert_eq!(state.reply.as_ref().unwrap().text, text);
    assert_eq!(state.command(101).reply.unwrap().id, receipt.reply_id);
    assert_eq!(state.seq, 0);
}
