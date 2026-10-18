use super::{Error, MAX_BUFFER_BYTES, SseBuffer};

#[test]
fn assembles_fragmented_task_events() {
    let mut buffer = SseBuffer::new();
    let first = buffer
        .push(br#"data: {"task_id":"task-1","message":"hel"#)
        .unwrap();
    assert!(first.is_empty());

    let tasks = buffer
        .push(&[b'l', b'o', b'"', b'}', b'\n', b'\n'])
        .unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].task_id, "task-1");
    assert_eq!(tasks[0].message, "hello");
}

#[test]
fn rejects_delimiter_free_oversized_frames() {
    let mut buffer = SseBuffer::new();
    let error = buffer.push(&vec![b'x'; MAX_BUFFER_BYTES + 1]).unwrap_err();
    assert!(matches!(error, Error::Oversized));
}

#[test]
fn rejects_invalid_utf8() {
    let error = SseBuffer::new().push(&[0xff]).unwrap_err();
    assert!(matches!(error, Error::InvalidUtf8));
}
