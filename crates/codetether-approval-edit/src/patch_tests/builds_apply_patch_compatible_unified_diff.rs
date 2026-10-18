//! Existing patch-generation regression case.
use crate::ApprovalEditFile;
use std::path::PathBuf;

#[test]
fn builds_apply_patch_compatible_unified_diff() {
    let files = vec![ApprovalEditFile {
        path: PathBuf::from("src/lib.rs"),
        relative: "src/lib.rs".into(),
        original: "fn old() {}\n".into(),
        revised: "fn edited() {}\n".into(),
    }];

    let patch = crate::patch::build(&files);

    assert!(patch.starts_with("--- a/src/lib.rs\n+++ b/src/lib.rs\n"));
    assert!(patch.contains("-fn old() {}"));
    assert!(patch.contains("+fn edited() {}"));
}
