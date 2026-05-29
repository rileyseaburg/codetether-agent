use crate::tui::app::file_picker::types::FilePickerMode;
use crate::tui::app::file_picker::{
    file_picker_enter, file_picker_escape, file_picker_page_down, open_file_picker,
};
use crate::tui::app::state::App;

#[test]
fn escape_exits_viewer_first() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("note.txt"), "hello").expect("file");
    let mut app = App::default();
    open_file_picker(&mut app, dir.path());
    app.state.file_picker.selected = app
        .state
        .file_picker
        .entries
        .iter()
        .position(|entry| entry.name == "note.txt")
        .expect("entry exists");
    file_picker_enter(&mut app, dir.path());

    assert!(file_picker_escape(&mut app));
    assert_eq!(app.state.file_picker.mode, FilePickerMode::Browse);
}

#[test]
fn page_down_clamps_to_preview_length() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("note.txt");
    std::fs::write(&path, "one\ntwo\nthree").expect("file");
    let mut app = App::default();
    open_file_picker(&mut app, dir.path());
    app.state.file_picker.selected = app
        .state
        .file_picker
        .entries
        .iter()
        .position(|entry| entry.name == "note.txt")
        .expect("entry exists");
    file_picker_enter(&mut app, dir.path());

    file_picker_page_down(&mut app);
    file_picker_page_down(&mut app);

    assert_eq!(app.state.file_picker.preview_scroll, 2);
}
