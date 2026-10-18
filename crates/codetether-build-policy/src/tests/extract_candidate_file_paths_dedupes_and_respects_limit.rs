//! Original build-policy regression case.
use crate::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn extract_candidate_file_paths_dedupes_and_respects_limit() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("a")).expect("mkdirs");
    fs::create_dir_all(root.join("b")).expect("mkdirs");
    fs::write(root.join("a/one.ts"), "export {};").expect("write file");
    fs::write(root.join("b/two.tsx"), "export {};").expect("write file");

    let text = "a/one.ts a/one.ts b/two.tsx";
    let paths = extract_candidate_file_paths(text, root, 1);
    assert_eq!(paths, vec!["a/one.ts".to_string()]);
}
