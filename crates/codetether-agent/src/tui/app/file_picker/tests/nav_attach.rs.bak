use crate::tui::app::file_picker::{file_picker_attach, open_file_picker};
use crate::tui::app::state::App;

#[test]
fn attach_adds_text_file_to_composer() {
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

    file_picker_attach(&mut app);

    assert!(app.state.input.contains("Shared file: note.txt"));
}
