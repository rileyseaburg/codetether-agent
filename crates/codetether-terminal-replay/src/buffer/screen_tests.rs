//! Attach-time screen-state and bounded replay regressions.
use super::OutputBuffer;

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
