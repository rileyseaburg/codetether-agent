//! Regression cases for the parent stream module.

use super::EventId;

#[test]
fn roundtrip() {
    let id = EventId::parse("abc.99").unwrap();
    assert_eq!(id.seq, 99);
    assert_eq!(id.format(), "abc.99");
}

#[test]
fn epoch_with_dots_keeps_last_as_separator() {
    let id = EventId::parse("a.b.c.7").unwrap();
    assert_eq!(id.epoch, "a.b.c");
    assert_eq!(id.seq, 7);
}

#[test]
fn rejects_malformed() {
    assert!(EventId::parse("noseq").is_none());
    assert!(EventId::parse(".5").is_none());
    assert!(EventId::parse("e.x").is_none());
    assert!(EventId::parse("e.-1").is_none());
}
