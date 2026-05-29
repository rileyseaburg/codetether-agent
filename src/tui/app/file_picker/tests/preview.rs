use crate::tui::app::file_picker::preview::load_preview;

#[test]
fn binary_preview_reports_metadata_only() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("blob.bin");
    std::fs::write(&path, [0_u8, 1, 2]).expect("binary file");

    let preview = load_preview(&path);

    assert!(preview.binary);
    assert!(preview.lines[0].contains("Binary file:"));
}

#[test]
fn large_text_preview_reports_truncation() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("large.txt");
    std::fs::write(&path, "x".repeat(70 * 1024)).expect("large file");

    let preview = load_preview(&path);

    assert!(preview.truncated);
    assert!(!preview.binary);
}

#[test]
fn directory_preview_is_clean() {
    let dir = tempfile::tempdir().expect("tempdir");

    let preview = load_preview(dir.path());

    assert!(!preview.binary);
    assert!(preview.lines[0].starts_with("Directory:"));
}
