use crate::tui::app::file_picker::file_picker_filter_push;
use crate::tui::app::state::App;

#[test]
fn filter_accepts_letter_a() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("alpha.txt"), "a").expect("file");
    let mut app = App::default();
    crate::tui::app::file_picker::open_file_picker(&mut app, dir.path());

    file_picker_filter_push(&mut app, 'a');

    assert_eq!(app.state.file_picker.filter, "a");
    assert!(
        app.state
            .file_picker
            .entries
            .iter()
            .any(|e| e.name == "alpha.txt")
    );
}
