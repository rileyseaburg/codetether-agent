//! Regression cases for the parent stream module.

use super::{StagingBuffer, StagingError};

#[test]
fn accepts_within_cap() {
    let mut buf = StagingBuffer::new(8);
    assert!(buf.extend(b"abcd").is_ok());
    assert!(buf.extend(b"efgh").is_ok());
    assert_eq!(buf.bytes_mut().len(), 8);
}

#[test]
fn rejects_overflow() {
    let mut buf = StagingBuffer::new(4);
    assert_eq!(
        buf.extend(b"abcde"),
        Err(StagingError::Overflow { max_bytes: 4 })
    );
}

#[test]
fn drain_lowers_occupancy_allowing_more() {
    let mut buf = StagingBuffer::new(4);
    buf.extend(b" abcd"[1..].as_ref()).unwrap();
    buf.bytes_mut().drain(..2);
    assert!(buf.extend(b"xy").is_ok());
}
