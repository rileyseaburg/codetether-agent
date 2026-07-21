//! Patch-format target extraction tests.

use serde_json::json;
use std::path::PathBuf;

#[test]
fn apply_patch_markers_produce_exact_targets() {
    let input = json!({
        "patch": "*** Begin Patch\n*** Update File: /repo/src/lib.rs\n*** Add File: /repo/src/new.rs\n*** End Patch"
    });
    let paths = super::patch::paths(&input).unwrap();
    assert_eq!(
        paths,
        vec![
            PathBuf::from("/repo/src/lib.rs"),
            PathBuf::from("/repo/src/new.rs")
        ]
    );
}
