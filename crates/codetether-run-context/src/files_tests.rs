use super::with_attachments;
use std::{fs, path::PathBuf};

#[test]
fn preserves_message_without_files() {
    assert_eq!(with_attachments("repair", &[]).unwrap(), "repair");
}

#[test]
fn includes_canonical_attachment_path() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("failure.json");
    fs::write(&file, "{}").unwrap();

    let message = with_attachments("repair", std::slice::from_ref(&file)).unwrap();

    assert!(message.starts_with("repair\n\nAttached files"));
    assert!(message.contains(file.canonicalize().unwrap().to_str().unwrap()));
    assert!(message.contains("use these exact paths"));
}

#[test]
fn rejects_missing_attachment() {
    let missing = PathBuf::from("missing-attachment-for-run-test.json");

    let error = with_attachments("repair", &[missing]).unwrap_err();

    assert!(error.to_string().contains("Attached file does not exist"));
}

#[test]
fn rejects_directory_attachment() {
    let dir = tempfile::tempdir().unwrap();

    let error = with_attachments("repair", &[dir.path().to_path_buf()]).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("Attachment is not a regular file")
    );
}
