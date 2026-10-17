//! Existing framing regression.
use super::*;

#[test]
fn parses_single_frame_with_headers() {
    let frame = build_frame(
        &[(":event-type", "messageStart"), (":message-type", "event")],
        br#"{"role":"assistant"}"#,
    );
    let mut buf = FrameBuffer::new();
    buf.extend(&frame);
    let msg = buf.next_frame().unwrap().unwrap();
    assert_eq!(msg.event_type(), Some("messageStart"));
    assert_eq!(msg.message_type(), Some("event"));
    assert_eq!(msg.payload, br#"{"role":"assistant"}"#);
}
