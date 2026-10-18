//! Existing patch-generation regression case.
use crate::ApprovalEditFile;
use std::path::PathBuf;

#[test]
fn generated_patch_replays_through_patch_runtime() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("src/lib.rs");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "fn old() {}").unwrap();
    let files = vec![ApprovalEditFile {
        path: path.clone(),
        relative: "src/lib.rs".into(),
        original: "fn old() {}".into(),
        revised: "fn user_edited() {}".into(),
    }];

    let patch = crate::patch::build(&files);
    let proposed = codetether_patch::proposed::contents(dir.path(), &patch).unwrap();

    assert_eq!(proposed, vec![(path, "fn user_edited() {}".into())]);
}
