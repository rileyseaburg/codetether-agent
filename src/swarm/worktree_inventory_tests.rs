use super::super::inventory_parse::changed_path;

#[test]
fn parses_added_modified_and_untracked_paths() {
    assert_eq!(changed_path(" M src/lib.rs").as_deref(), Some("src/lib.rs"));
    assert_eq!(
        changed_path("?? src/resp/mod.rs").as_deref(),
        Some("src/resp/mod.rs")
    );
    assert_eq!(changed_path("A  src/new.rs").as_deref(), Some("src/new.rs"));
}

#[test]
fn resolves_rename_to_destination_and_skips_ignored() {
    assert_eq!(
        changed_path("R  old.rs -> new.rs").as_deref(),
        Some("new.rs")
    );
    assert_eq!(changed_path("!! target/debug"), None);
    assert_eq!(changed_path(""), None);
}

#[tokio::test]
async fn inventories_files_a_blocked_agent_wrote() {
    let dir = tempfile::tempdir().unwrap();
    for args in [
        vec!["init", "-q"],
        vec!["config", "user.email", "a@b.c"],
        vec!["config", "user.name", "test"],
    ] {
        tokio::process::Command::new("git")
            .args(&args)
            .current_dir(dir.path())
            .output()
            .await
            .unwrap();
    }
    std::fs::create_dir_all(dir.path().join("src/resp")).unwrap();
    std::fs::write(dir.path().join("src/resp/codec.rs"), "// codec\n").unwrap();
    std::fs::write(dir.path().join("src/resp/mod.rs"), "// mod\n").unwrap();

    let files = super::written_files(dir.path()).await;

    assert_eq!(files, vec!["src/resp/codec.rs", "src/resp/mod.rs"]);
}
