use super::{implicit, resolved};
use std::path::{Path, PathBuf};

#[test]
fn implicit_roots_include_sandbox_home_temp_and_work_dir() {
    let roots = implicit(Path::new("/workspace"), Path::new("/var/folders/xy"));
    assert!(roots.contains(&PathBuf::from("/tmp")), "sandbox HOME");
    assert!(roots.contains(&PathBuf::from("/var/folders/xy")));
    assert!(roots.contains(&PathBuf::from("/workspace")));
}

#[test]
fn resolved_emits_both_symlink_and_target_paths() {
    let dir = tempfile::tempdir().expect("temp dir");
    let target = dir.path().join("real");
    std::fs::create_dir(&target).expect("create target");
    let link = dir.path().join("link");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, &link).expect("symlink");

    #[cfg(unix)]
    {
        let paths = resolved(&link);
        assert!(paths.contains(&link), "original path must stay in the rules");
        assert_eq!(paths.len(), 2, "resolved target must be added: {paths:?}");
    }
}

#[test]
fn resolved_keeps_nonexistent_paths_verbatim() {
    let missing = PathBuf::from("/definitely/not/here");
    assert_eq!(resolved(&missing), vec![missing.clone()]);
}