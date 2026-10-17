//! Existing framing regression.
use super::*;

#[test]
fn handles_chunked_delivery() {
    let frame = build_frame(&[(":event-type", "x")], b"hello");
    let mut buf = FrameBuffer::new();
    buf.extend(&frame[..5]);
    assert!(buf.next_frame().unwrap().is_none());
    buf.extend(&frame[5..]);
    assert!(buf.next_frame().unwrap().is_some());
}
