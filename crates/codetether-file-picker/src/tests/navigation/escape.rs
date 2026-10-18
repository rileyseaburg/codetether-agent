use crate::FilePickerMode;
use crate::{file_picker_enter, file_picker_escape, file_picker_page_down, open_file_picker};

#[test]
fn escape_exits_viewer_first() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("note.txt"), "hello").expect("file");
    let mut picker = open_file_picker(dir.path());
    picker.selected = picker
        .entries
        .iter()
        .position(|entry| entry.name == "note.txt")
        .expect("entry exists");
    file_picker_enter(&mut picker, dir.path());

    assert!(file_picker_escape(&mut picker));
    assert_eq!(picker.mode, FilePickerMode::Browse);
}

#[test]
fn page_down_clamps_to_preview_length() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("note.txt");
    std::fs::write(&path, "one\ntwo\nthree").expect("file");
    let mut picker = open_file_picker(dir.path());
    picker.selected = picker
        .entries
        .iter()
        .position(|entry| entry.name == "note.txt")
        .expect("entry exists");
    file_picker_enter(&mut picker, dir.path());

    file_picker_page_down(&mut picker);
    file_picker_page_down(&mut picker);

    assert_eq!(picker.preview_scroll, 2);
}
