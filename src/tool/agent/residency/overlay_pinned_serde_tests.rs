//! Workspace pins roundtrip while historical metadata remains unpinned.

use crate::session::SessionMetadata;
use std::path::PathBuf;

#[test]
fn workspace_pin_roundtrips_with_its_directory() {
    for pinned in [true, false] {
        let metadata = SessionMetadata {
            directory: Some(PathBuf::from("/workspace/.codetether-worktrees/child")),
            workspace_pinned: pinned,
            ..Default::default()
        };
        let json = serde_json::to_value(&metadata).unwrap();
        assert_eq!(json["workspace_pinned"], pinned);
        let restored: SessionMetadata = serde_json::from_value(json).unwrap();
        assert_eq!(restored.workspace_pinned, pinned);
        assert_eq!(restored.directory, metadata.directory);
    }
}

#[test]
fn legacy_metadata_without_workspace_pin_defaults_to_unpinned() {
    let mut json = serde_json::to_value(SessionMetadata::default()).unwrap();
    json.as_object_mut().unwrap().remove("workspace_pinned");
    let restored: SessionMetadata = serde_json::from_value(json).unwrap();
    assert!(!restored.workspace_pinned);
}
