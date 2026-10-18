use crate::{file_picker_filter_push, open_file_picker};

#[test]
fn filter_accepts_letter_a() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("alpha.txt"), "a").expect("file");
    let mut picker = open_file_picker(dir.path());

    file_picker_filter_push(&mut picker, 'a');

    assert_eq!(picker.filter, "a");
    assert!(picker.entries.iter().any(|e| e.name == "alpha.txt"));
}
