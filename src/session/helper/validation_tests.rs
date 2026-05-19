use super::validation::*;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[test]
fn tracks_edit_paths_from_arguments() {
    let workspace_dir = Path::new("/workspace");
    let mut touched_files = HashSet::new();
    track_touched_files(
        &mut touched_files,
        workspace_dir,
        "edit",
        &json!({ "path": "src/main.ts" }),
        None,
    );

    assert!(touched_files.contains(&PathBuf::from("/workspace/src/main.ts")));
}

#[test]
fn tracks_patch_paths_from_metadata() {
    let workspace_dir = Path::new("/workspace");
    let mut touched_files = HashSet::new();
    let metadata = HashMap::from([("files".to_string(), json!(["src/lib.rs", "tests/app.rs"]))]);

    track_touched_files(
        &mut touched_files,
        workspace_dir,
        "patch",
        &json!({}),
        Some(&metadata),
    );

    assert!(touched_files.contains(&PathBuf::from("/workspace/src/lib.rs")));
    assert!(touched_files.contains(&PathBuf::from("/workspace/tests/app.rs")));
}
