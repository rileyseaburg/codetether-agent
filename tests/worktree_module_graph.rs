use std::path::Path;

#[test]
fn worktree_uses_single_active_module_file() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let monolith = root.join("src/worktree.rs");
    let split_mod = root.join("src/worktree/mod.rs");
    let split_dir = root.join("src/worktree");
    let has_monolith = monolith.is_file();
    let has_split_mod = split_mod.is_file();

    assert!(
        has_monolith || has_split_mod,
        "expected either src/worktree.rs or src/worktree/mod.rs to define the worktree module"
    );
    assert!(
        !(has_monolith && has_split_mod),
        "expected exactly one active worktree module entry point"
    );
    if !split_dir.is_dir() || has_split_mod {
        return;
    }

    assert!(
        !contains_rust_file(&split_dir),
        "src/worktree/ contains Rust modules, but src/worktree.rs is the active module"
    );
}

fn contains_rust_file(path: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(path) else {
        return false;
    };
    entries.filter_map(Result::ok).any(|entry| {
        let path = entry.path();
        path.extension().is_some_and(|ext| ext == "rs")
            || (path.is_dir() && contains_rust_file(&path))
    })
}
