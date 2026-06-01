use crate::tui::app::file_picker::types::FilePickerMode;
use crate::tui::app::file_picker::{
    file_picker_enter, file_picker_select_first, file_picker_select_last, open_file_picker,
};
use crate::tui::app::state::App;

#[test]
fn enter_opens_file_viewer() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("note.txt"), "hello").expect("file");
    let mut app = App::default();
    open_file_picker(&mut app, dir.path());
    select_name(&mut app, "note.txt");

    file_picker_enter(&mut app, dir.path());

    assert_eq!(app.state.file_picker.mode, FilePickerMode::Viewer);
    assert!(app.state.file_picker.preview.is_some());
}

#[test]
fn enter_navigates_directories() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir(dir.path().join("src")).expect("dir");
    let mut app = App::default();
    open_file_picker(&mut app, dir.path());
    select_name(&mut app, "src/");

    file_picker_enter(&mut app, dir.path());

    assert_eq!(app.state.file_picker.dir, dir.path().join("src"));
}

#[test]
fn home_end_jump_to_visible_extremes() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("a.txt"), "a").expect("file");
    std::fs::write(dir.path().join("z.txt"), "z").expect("file");
    let mut app = App::default();
    open_file_picker(&mut app, dir.path());

    file_picker_select_last(&mut app);
    assert_eq!(
        app.state.file_picker.entries[app.state.file_picker.selected].name,
        "z.txt"
    );
    file_picker_select_first(&mut app);
    assert_eq!(app.state.file_picker.selected, 0);
}

fn select_name(app: &mut App, name: &str) {
    app.state.file_picker.selected = app
        .state
        .file_picker
        .entries
        .iter()
        .position(|entry| entry.name == name)
        .expect("entry exists");
}
