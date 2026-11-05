//! Regression coverage for incremental SSE frame boundaries.
use super::frame_end;

#[test]
fn framing_accepts_lf_and_crlf_and_waits_for_complete_delimiters() {
    assert_eq!(frame_end(b""), None);
    assert_eq!(frame_end(b"data: {}\n"), None);
    assert_eq!(frame_end(b"data: {}\r\n\r"), None);
    assert_eq!(frame_end(b"data: {}\n\n"), Some(10));
    assert_eq!(frame_end(b"data: {}\r\n\r\n"), Some(12));
    assert_eq!(frame_end(b"\n\nnext\r\n\r\n"), Some(2));
}
