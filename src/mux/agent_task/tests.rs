use super::{buffer::TaskBuffer, validation};

#[test]
fn replay_reads_structured_output_from_requested_offset() {
    let mut buffer = TaskBuffer::new();
    buffer.append(b"one\ntwo\n");

    let (data, offset) = buffer.read(4);

    assert_eq!(data, b"two\n");
    assert_eq!(offset, 8);
}

#[test]
fn validation_rejects_unbounded_or_unsafe_tasks() {
    assert!(validation::request("task-1234", "hello", None, 1).is_ok());
    assert!(validation::request("bad", "hello", None, 1).is_err());
    assert!(validation::request("task-1234", "", None, 1).is_err());
    assert!(validation::request("task-1234", "hello", None, 251).is_err());
}
