use crate::{FilePickerMode, FilePickerState};
use crate::{
    file_picker_enter, file_picker_select_first, file_picker_select_last, open_file_picker,
};

#[test]
fn enter_opens_file_viewer() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("note.txt"), "hello").expect("file");
    let mut picker = open_file_picker(dir.path());
    select_name(&mut picker, "note.txt");

    file_picker_enter(&mut picker, dir.path());

    assert_eq!(picker.mode, FilePickerMode::Viewer);
    assert!(picker.preview.is_some());
}

#[test]
fn enter_navigates_directories() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir(dir.path().join("src")).expect("dir");
    let mut picker = open_file_picker(dir.path());
    select_name(&mut picker, "src/");

    file_picker_enter(&mut picker, dir.path());

    assert_eq!(picker.dir, dir.path().join("src"));
}

#[test]
fn home_end_jump_to_visible_extremes() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("a.txt"), "a").expect("file");
    std::fs::write(dir.path().join("z.txt"), "z").expect("file");
    let mut picker = open_file_picker(dir.path());

    file_picker_select_last(&mut picker);
    assert_eq!(picker.entries[picker.selected].name, "z.txt");
    file_picker_select_first(&mut picker);
    assert_eq!(picker.selected, 0);
}

fn select_name(picker: &mut FilePickerState, name: &str) {
    picker.selected = picker
        .entries
        .iter()
        .position(|entry| entry.name == name)
        .expect("entry exists");
}
