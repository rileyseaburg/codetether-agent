use super::{path_entries_from, roots_from};
use std::ffi::OsString;
use std::path::PathBuf;

#[test]
fn roots_include_existing_defaults_and_configured_paths_once() {
    let home = tempfile::tempdir().expect("home");
    let node = home.path().join(".nvm/versions/node");
    std::fs::create_dir_all(&node).expect("node dir");
    let extra = tempfile::tempdir().expect("extra");
    let configured = std::env::join_paths([extra.path(), extra.path(), node.as_path()])
        .expect("join");

    let roots = roots_from(Some(&configured), Some(home.path()));

    assert_eq!(roots, vec![extra.path().to_path_buf(), node]);
}

#[test]
fn roots_skip_missing_and_relative_entries() {
    let home = tempfile::tempdir().expect("home");
    let configured = OsString::from("relative/tools:/definitely/missing/root");

    assert!(roots_from(Some(&configured), Some(home.path())).is_empty());
    assert!(roots_from(None, None).is_empty());
}

#[test]
fn path_entries_keep_host_order_for_visible_roots_only() {
    let root = PathBuf::from("/opt/toolchain");
    let host = std::env::join_paths([
        "/opt/toolchain/node/bin",
        "/home/user/secret/bin",
        "/usr/local/bin",
        "/opt/toolchain/node/bin",
        "/bin",
    ])
    .expect("join");

    let entries = path_entries_from(&host, std::slice::from_ref(&root));

    assert_eq!(
        entries,
        vec![
            PathBuf::from("/opt/toolchain/node/bin"),
            PathBuf::from("/usr/local/bin"),
            PathBuf::from("/bin"),
        ]
    );
}
