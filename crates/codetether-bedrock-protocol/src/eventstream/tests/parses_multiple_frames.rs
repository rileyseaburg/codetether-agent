//! Existing framing regression.
use super::*;

#[test]
fn parses_multiple_frames() {
    let mut all = Vec::new();
    all.extend(build_frame(&[(":event-type", "a")], b"1"));
    all.extend(build_frame(&[(":event-type", "b")], b"22"));
    let mut buf = FrameBuffer::new();
    buf.extend(&all);
    assert_eq!(buf.next_frame().unwrap().unwrap().event_type(), Some("a"));
    assert_eq!(buf.next_frame().unwrap().unwrap().event_type(), Some("b"));
    assert!(buf.next_frame().unwrap().is_none());
}
