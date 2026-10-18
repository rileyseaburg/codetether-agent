//! Existing patch-generation regression case.
use crate::ApprovalEditFile;
use std::path::PathBuf;

#[test]
fn omits_files_reverted_to_original_content() {
    let files = vec![ApprovalEditFile {
        path: PathBuf::from("src/lib.rs"),
        relative: "src/lib.rs".into(),
        original: "same\n".into(),
        revised: "same\n".into(),
    }];
    assert!(crate::patch::build(&files).is_empty());
}
