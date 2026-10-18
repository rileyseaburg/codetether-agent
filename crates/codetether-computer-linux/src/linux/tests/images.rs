//! Reject invalid or oversized captures before attaching image data.

use super::*;
use codetether_computer_capture::limits::MAX_FILE_BYTES;

#[test]
fn png_originals_are_checked_and_preserved() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("desktop.png");
    let bytes = png();
    std::fs::write(&path, &bytes).unwrap();
    let (original, width, height) = image_file::load(&path).unwrap();
    assert_eq!((width, height), (80, 40));
    assert_eq!(original, bytes);
    for bad in [&b"not an image"[..], &bytes[..bytes.len() / 2], &[]] {
        std::fs::write(&path, bad).unwrap();
        assert!(image_file::load(&path).is_err());
    }
}

#[test]
fn oversized_files_are_rejected_before_reading() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("too-large.png");
    let file = std::fs::File::create(&path).unwrap();
    file.set_len(MAX_FILE_BYTES + 1).unwrap();
    let error = image_file::load(&path).unwrap_err();
    assert!(error.to_string().contains("safety budget"));
}
