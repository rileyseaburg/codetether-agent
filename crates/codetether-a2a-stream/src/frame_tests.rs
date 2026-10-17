//! Regression cases for the parent stream module.

use super::parse_frame;

#[test]
fn no_id_defaults_message_type_when_absent() {
    let f = parse_frame("data: hello").unwrap();
    assert!(f.id.is_none());
    assert_eq!(f.event, "message");
    assert_eq!(f.data, "hello");
}

#[test]
fn returns_none_without_data() {
    assert!(parse_frame("event: heartbeat").is_none());
}
