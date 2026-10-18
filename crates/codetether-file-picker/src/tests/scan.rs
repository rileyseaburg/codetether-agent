use crate::{FilePickerState, filter::rescan_with_filter, scan::scan_directory};

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
    let all_entries = scan_directory(dir.path());
    let mut state = FilePickerState {
        dir: dir.path().to_path_buf(),
        all_entries,
        filter: "missing".to_string(),
        ..Default::default()
    };

    rescan_with_filter(&mut state);

    assert_eq!(state.entries.len(), 1);
    assert_eq!(state.entries[0].name, "../");
}
