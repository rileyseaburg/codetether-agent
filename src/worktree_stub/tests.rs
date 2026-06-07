use super::inject;

#[test]
fn skips_when_no_cargo_toml() {
    let dir = tempfile::tempdir().expect("tempdir");
    inject(dir.path(), dir.path()).expect("non-Rust worktree should be a no-op");
}

#[test]
fn idempotent_when_workspace_already_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let cargo_toml = dir.path().join("Cargo.toml");
    std::fs::write(&cargo_toml, "[workspace]\n[package]\nname = \"x\"\n").expect("write");
    inject(dir.path(), dir.path()).expect("idempotent");
    let contents = std::fs::read_to_string(&cargo_toml).expect("read");
    let occurrences = contents.matches("[workspace]").count();
    assert_eq!(occurrences, 1, "stub must not be duplicated");
}

#[test]
fn adds_worktree_target_config() {
    let dir = tempfile::tempdir().expect("tempdir");
    let worktree = dir.path().join("wt-one");
    std::fs::create_dir(&worktree).expect("worktree");
    std::fs::write(worktree.join("Cargo.toml"), "[package]\nname = \"x\"\n").expect("write");
    inject(&worktree, dir.path()).expect("target config");
    let config = std::fs::read_to_string(worktree.join(".cargo/config.toml")).expect("read");
    assert!(config.contains("[build]"));
    assert!(config.contains(".targets/wt-one"));
}
