//! Tests for [`super::cursor::Cursor`].

use super::cursor::Cursor;
use super::event_id::EventId;

#[test]
fn commit_persists_and_reloads() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("c");
    let mut cursor = Cursor::load(path.clone());
    assert!(cursor.last().is_none());
    cursor
        .commit(&EventId { epoch: "ep".into(), seq: 3 })
        .unwrap();
    assert_eq!(Cursor::load(path).last().unwrap().seq, 3);
}

#[test]
fn reset_clears_memory_and_disk() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("c");
    let mut cursor = Cursor::load(path.clone());
    cursor
        .commit(&EventId { epoch: "ep".into(), seq: 9 })
        .unwrap();
    cursor.reset().unwrap();
    assert!(cursor.last().is_none());
    assert!(Cursor::load(path).last().is_none());
}

#[test]
fn reset_is_idempotent_when_absent() {
    let dir = tempfile::tempdir().unwrap();
    let mut cursor = Cursor::load(dir.path().join("missing"));
    cursor.reset().unwrap();
    assert!(cursor.last().is_none());
}
