use super::{OUTPUT_LIMIT, OutputBuffer};

#[test]
fn capacity_never_grows_past_replay_limit() {
    let mut buffer = OutputBuffer::new();
    let chunk = vec![b'x'; 8192];
    for _ in 0..(OUTPUT_LIMIT / chunk.len() * 3) {
        buffer.append(&chunk);
    }
    assert_eq!(buffer.bytes.len(), OUTPUT_LIMIT);
    assert!(buffer.bytes.capacity() <= OUTPUT_LIMIT);
}

#[test]
fn oversized_append_keeps_latest_replay_bytes() {
    let mut buffer = OutputBuffer::new();
    buffer.append(&vec![b'x'; OUTPUT_LIMIT + 17]);
    let (replay, next) = buffer.read(0);
    assert_eq!(buffer.earliest(), 17);
    assert_eq!(replay.len(), super::READ_LIMIT);
    assert_eq!(next, 17 + super::READ_LIMIT as u64);
}

#[test]
fn alternate_screen_reconnect_starts_at_live_tail() {
    let mut buffer = OutputBuffer::new();
    buffer.append(b"before\x1b[?10");
    buffer.append(b"49hhistoric-redraws");
    let (offset, alternate_screen) = buffer.attach_state();
    assert!(alternate_screen);
    assert_eq!(offset, b"before\x1b[?1049hhistoric-redraws".len() as u64);
}

#[test]
fn normal_screen_reconnect_replays_buffered_output() {
    let mut buffer = OutputBuffer::new();
    buffer.append(b"\x1b[?1049hframe\x1b[?1049lprompt");
    let (offset, alternate_screen) = buffer.attach_state();
    assert!(!alternate_screen);
    assert_eq!(offset, buffer.earliest());
}
