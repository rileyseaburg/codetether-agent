//! Integration tests for the file-backed editor buffer.
//!
//! Exercises the open → edit → save round-trip against a real temp file.

use codetether_agent::tui::ui::editor::{EditorEdit, FileBuffer};

#[test]
fn open_edit_save_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("note.txt");
    std::fs::write(&path, "hello\n").unwrap();

    let mut buf = FileBuffer::open(&path).unwrap();
    assert!(!buf.is_dirty());

    // Insert 'X' at the start: "Xhello\n".
    buf.backend_mut().insert_char('X');
    assert!(buf.is_dirty());

    buf.save().unwrap();
    assert!(!buf.is_dirty());

    let on_disk = std::fs::read_to_string(&path).unwrap();
    assert_eq!(on_disk, "Xhello\n");
}

#[test]
fn open_missing_file_starts_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("new.txt");

    let mut buf = FileBuffer::open(&path).unwrap();
    buf.backend_mut().insert_char('a');
    buf.save().unwrap();

    assert_eq!(std::fs::read_to_string(&path).unwrap(), "a");
}
