use crate::session::SessionEvent;

use super::mapper;

#[test]
fn retry_abandons_partial_item_before_next_attempt() {
    let mut mapper = mapper();
    let first = mapper.map_session_event(&SessionEvent::TextChunk("partial".into()));
    let first_id = first[0].payload["item_id"].clone();
    let retry = mapper.map_session_event(&SessionEvent::StreamRetry(
        crate::session::StreamRetryEvent {
            attempt: 1,
            max_restarts: 3,
            reason: "connection reset".into(),
        },
    ));
    let second = mapper.map_session_event(&SessionEvent::TextChunk("complete".into()));
    assert_eq!(retry[0].kind, "stream.retry");
    assert_eq!(retry[0].payload["abandoned_item_id"], first_id);
    assert_eq!(second[0].kind, "item.started");
    assert_ne!(second[0].payload["item_id"], first_id);
}
