use std::path::PathBuf;

use super::super::ApprovalEditFile;

#[test]
fn builds_apply_patch_compatible_unified_diff() {
    let files = vec![ApprovalEditFile {
        path: PathBuf::from("src/lib.rs"),
        relative: "src/lib.rs".into(),
        original: "fn old() {}\n".into(),
        revised: "fn edited() {}\n".into(),
    }];

    let patch = super::build(&files);

    assert!(patch.starts_with("--- a/src/lib.rs\n+++ b/src/lib.rs\n"));
    assert!(patch.contains("-fn old() {}"));
    assert!(patch.contains("+fn edited() {}"));
}

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

    let patch = super::build(&files);
    let proposed = crate::tool::patch::proposed::contents(dir.path(), &patch).unwrap();

    assert_eq!(proposed, vec![(path, "fn user_edited() {}".into())]);
}

#[test]
fn omits_files_reverted_to_original_content() {
    let files = vec![ApprovalEditFile {
        path: PathBuf::from("src/lib.rs"),
        relative: "src/lib.rs".into(),
        original: "same\n".into(),
        revised: "same\n".into(),
    }];
    assert!(super::build(&files).is_empty());
}
