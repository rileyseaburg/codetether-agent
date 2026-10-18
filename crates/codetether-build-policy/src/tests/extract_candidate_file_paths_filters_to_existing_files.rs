//! Original build-policy regression case.
use crate::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn extract_candidate_file_paths_filters_to_existing_files() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("api/src")).expect("mkdirs");
    fs::write(root.join("api/src/index.ts"), "export {};").expect("write file");

    let text = "Check api/src/index.ts and missing/path.ts";
    let paths = extract_candidate_file_paths(text, root, 5);
    assert_eq!(paths, vec!["api/src/index.ts".to_string()]);
}
