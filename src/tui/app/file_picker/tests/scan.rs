use crate::tui::app::file_picker::file_picker_filter_push;
use crate::tui::app::file_picker::filter::rescan_with_filter;
use crate::tui::app::file_picker::scan::scan_directory;
use crate::tui::app::file_picker::types::FilePickerState;
use crate::tui::app::state::App;

#[test]
fn scan_sorts_parent_dirs_then_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir(dir.path().join("alpha")).expect("dir");
    std::fs::write(dir.path().join("beta.txt"), "b").expect("file");

    let names = scan_directory(dir.path())
        .into_iter()
        .map(|entry| entry.name)
        .collect::<Vec<_>>();

    assert_eq!(&names[..3], &["../", "alpha/", "beta.txt"]);
}

#[test]
fn filter_preserves_parent_entry() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("beta.txt"), "b").expect("file");
    let mut state = FilePickerState {
        dir: dir.path().to_path_buf(),
        filter: "missing".to_string(),
        ..Default::default()
    };

    rescan_with_filter(&mut state);

    assert_eq!(state.entries.len(), 1);
    assert_eq!(state.entries[0].name, "../");
}

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
