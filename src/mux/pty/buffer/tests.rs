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
fn wrapped_replay_preserves_byte_order() {
    let mut buffer = OutputBuffer::new();
    buffer.append(&vec![b'x'; OUTPUT_LIMIT - 2]);
    buffer.append(b"abcdef");
    let (replay, _) = buffer.read(buffer.earliest() + OUTPUT_LIMIT as u64 - 6);
    assert_eq!(replay, b"abcdef");
}

#[test]
fn alternate_screen_attach_replays_current_buffer() {
    let mut buffer = OutputBuffer::new();
    buffer.append(b"before\x1b[?10");
    buffer.append(b"49hhistoric-redraws");
    let (offset, replay_until, alternate_screen) = buffer.attach_state();
    assert!(alternate_screen);
    assert_eq!(offset, buffer.earliest());
    assert_eq!(replay_until, buffer.latest());
    assert!(!buffer.read(offset).0.is_empty());
}

#[test]
fn normal_screen_attach_replay_is_bounded() {
    let mut buffer = OutputBuffer::new();
    buffer.append(&vec![b'x'; super::READ_LIMIT * 2]);
    let (offset, replay_until, alternate_screen) = buffer.attach_state();
    assert!(!alternate_screen);
    assert_eq!(replay_until - offset, super::READ_LIMIT as u64);
    assert_eq!(replay_until, buffer.latest());
}
