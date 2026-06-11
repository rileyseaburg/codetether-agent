use crate::session::SessionEvent;

use super::mapper;

#[test]
fn text_events_share_item_id() {
    let mut mapper = mapper();
    let chunk = mapper.map_session_event(&SessionEvent::TextChunk("hel".into()));
    let done = mapper.map_session_event(&SessionEvent::TextComplete("hello".into()));
    assert_eq!(chunk[0].kind, "item.started");
    assert_eq!(chunk[1].kind, "item.delta");
    assert_eq!(done[0].kind, "item.completed");
    assert_eq!(chunk[0].payload["item_id"], done[0].payload["item_id"]);
}
