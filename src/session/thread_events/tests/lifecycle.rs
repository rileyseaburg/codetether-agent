use super::mapper;

#[test]
fn turn_lifecycle_carries_stable_ids() {
    let mut mapper = mapper();
    let event = mapper.turn_started("hello");
    assert_eq!(event.event_id, "turn-1-event-000001");
    assert_eq!(event.thread_id, "session-1");
    assert_eq!(event.session_id, "session-1");
    assert_eq!(event.turn_id, "turn-1");
    assert_eq!(event.timestamp_ms, 99);
    assert_eq!(event.payload["message"], "hello");
}
