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
