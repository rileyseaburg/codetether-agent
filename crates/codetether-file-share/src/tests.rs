//! Existing binary-attachment regression case.
use crate::{FILE_SHARE_MAX_BYTES, build_file_share_snippet};

#[test]
fn binary_file_share_uses_metadata_only() {
    let dir = tempfile::tempdir().expect("tempdir should create");
    let path = dir.path().join("blob.bin");
    std::fs::write(&path, [0_u8, 1, 2, 3]).expect("binary fixture should write");

    let (snippet, truncated, binary) =
        build_file_share_snippet(&path, "blob.bin", FILE_SHARE_MAX_BYTES)
            .expect("snippet should build");

    assert!(binary);
    assert!(!truncated);
    assert!(snippet.contains("[binary file, size: 4B]"));
}
