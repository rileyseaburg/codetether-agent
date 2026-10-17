use super::{path_to_uri, workspace_folder::derive};

#[test]
fn derives_name_from_package_json() {
    let root = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        root.path().join("package.json"),
        r#"{"name":"spotless-web"}"#,
    )
    .expect("package.json");
    let uri = path_to_uri(root.path());
    let folders = derive(Some(uri.clone())).expect("folder");
    assert_eq!(folders.len(), 1);
    assert_eq!(folders[0].uri, uri);
    assert_eq!(folders[0].name, "spotless-web");
}

#[test]
fn falls_back_to_tsconfig_directory() {
    let root = tempfile::tempdir().expect("tempdir");
    std::fs::write(root.path().join("tsconfig.json"), "{}").expect("tsconfig");
    let folder = derive(Some(path_to_uri(root.path())))
        .expect("folder")
        .remove(0);
    assert_eq!(
        folder.name,
        root.path().file_name().unwrap().to_string_lossy()
    );
}

#[test]
fn absent_root_has_no_workspace_folder() {
    assert!(derive(None).is_none());
}
