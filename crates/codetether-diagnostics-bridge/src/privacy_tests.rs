//! Opaque payload and related-location boundary checks.
use crate::{Diagnostics, Publisher, Snapshot};

#[test]
fn related_locations_and_opaque_data_do_not_leak_other_workspaces() {
    let root = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let root_uri = url::Url::from_directory_path(root.path()).unwrap();
    let file_uri = root_uri.join("main.rs").unwrap();
    let diagnostic: lsp_types::Diagnostic = serde_json::from_value(serde_json::json!({
        "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 1}},
        "message": "diagnostic", "data": {"opaque": "must not be published"},
        "relatedInformation": [
            {"location": {"uri": file_uri.as_str(), "range": {"start": {"line":0,"character":0},"end":{"line":0,"character":1}}}, "message":"inside"},
            {"location": {"uri": "file:///unrelated/private.rs", "range": {"start": {"line":0,"character":0},"end":{"line":0,"character":1}}}, "message":"outside"}
        ]
    })).unwrap();
    let publisher = Publisher::in_directory(cache.path(), root_uri.as_str(), "server").unwrap();
    let data: Diagnostics = [(file_uri.to_string(), vec![diagnostic])]
        .into_iter()
        .collect();
    publisher.publish(&data).unwrap();
    let output: Snapshot =
        serde_json::from_slice(&std::fs::read(publisher.path()).unwrap()).unwrap();
    let diagnostic = &output.diagnostics[file_uri.as_str()][0];
    assert!(diagnostic.data.is_none());
    assert_eq!(diagnostic.related_information.as_ref().unwrap().len(), 1);
}

#[cfg(unix)]
#[test]
fn dangling_symlink_locations_are_excluded() {
    let root = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(
        "/nonexistent-external-diagnostics-directory",
        root.path().join("escape"),
    )
    .unwrap();
    let root_uri = url::Url::from_directory_path(root.path()).unwrap();
    let data = [(
        root_uri.join("escape/private.rs").unwrap().to_string(),
        Vec::new(),
    )]
    .into_iter()
    .collect();
    let publisher = Publisher::in_directory(cache.path(), root_uri.as_str(), "server").unwrap();
    publisher.publish(&data).unwrap();
    let output: Snapshot =
        serde_json::from_slice(&std::fs::read(publisher.path()).unwrap()).unwrap();
    assert!(output.diagnostics.is_empty());
}
