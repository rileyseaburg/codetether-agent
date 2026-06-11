use super::{RunEvent, event_value};

#[test]
fn item_started_has_id_and_timestamp() {
    let value = event_value(&RunEvent::ItemStarted {
        item_id: "item-1",
        timestamp_ms: 171,
    });
    assert_eq!(value["type"], "run.item_started");
    assert_eq!(value["item_id"], "item-1");
    assert_eq!(value["timestamp_ms"], 171);
}

#[test]
fn item_completed_has_id_and_timestamp() {
    let value = event_value(&RunEvent::ItemCompleted {
        item_id: "item-2",
        timestamp_ms: 172,
    });
    assert_eq!(value["type"], "run.item_completed");
    assert_eq!(value["item_id"], "item-2");
    assert_eq!(value["timestamp_ms"], 172);
}
