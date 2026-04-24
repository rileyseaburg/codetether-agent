use std::path::Path;

#[test]
fn worktree_uses_single_active_module_file() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let active = root.join("src/worktree.rs");
    let orphan_dir = root.join("src/worktree");

    assert!(active.is_file(), "expected active src/worktree.rs module");
    if !orphan_dir.exists() {
        return;
    }

    let orphan_modules = std::fs::read_dir(orphan_dir)
        .expect("read src/worktree")
        .filter_map(Result::ok)
        .any(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"));

    assert!(
        !orphan_modules,
        "src/worktree/ is not declared by src/lib.rs; wire modules or remove them"
    );
}
