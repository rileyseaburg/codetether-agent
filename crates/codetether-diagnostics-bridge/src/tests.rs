//! Snapshot replacement, root boundaries and private-file regression coverage.
use crate::{Diagnostics, Publisher, Snapshot};
use std::{fs, path::Path};
fn uri(path: &Path) -> String {
    url::Url::from_file_path(path).unwrap().into()
}
fn read(path: &Path) -> Snapshot {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn publish_clear_and_drop_are_producer_local() {
    let root = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let first = Publisher::in_directory(cache.path(), &uri(root.path()), "rust-analyzer").unwrap();
    let second = Publisher::in_directory(cache.path(), &uri(root.path()), "other").unwrap();
    assert_ne!(first.path(), second.path());
    let file = uri(&root.path().join("new file.rs"));
    let mut diagnostics = Diagnostics::new();
    diagnostics.insert(
        file.clone(),
        vec![lsp_types::Diagnostic::new_simple(
            Default::default(),
            "error".into(),
        )],
    );
    diagnostics.insert("file:///outside.rs".into(), Vec::new());
    diagnostics.insert("https://example.com/source.rs".into(), Vec::new());
    diagnostics.insert("not-a-uri".into(), Vec::new());
    first.publish(&diagnostics).unwrap();
    second.publish(&Diagnostics::new()).unwrap();
    let snapshot = read(first.path());
    assert_eq!(snapshot.schema_version, 1);
    assert_eq!(snapshot.pid, std::process::id());
    assert_eq!(snapshot.server, "rust-analyzer");
    assert_eq!(snapshot.diagnostics.len(), 1);
    assert_eq!(snapshot.diagnostics[&file].len(), 1);
    assert!(snapshot.workspace_uri.ends_with('/'));
    diagnostics.insert(file.clone(), Vec::new());
    first.publish(&diagnostics).unwrap();
    assert!(read(first.path()).diagnostics[&file].is_empty());
    first.publish(&Diagnostics::new()).unwrap();
    assert!(read(first.path()).diagnostics.is_empty());
    let filename = first.path().to_path_buf();
    drop(first);
    assert!(!filename.exists());
    assert!(second.path().exists());
    assert_eq!(fs::read_dir(cache.path()).unwrap().count(), 1);
}

#[test]
fn heartbeat_refresh_and_atomic_replacements_remain_readable() {
    let root = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let publisher = Publisher::in_directory(cache.path(), &uri(root.path()), "server").unwrap();
    publisher.publish(&Diagnostics::new()).unwrap();
    let previous = read(publisher.path()).updated_at_ms;
    std::thread::sleep(std::time::Duration::from_millis(3));
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0..50 {
                publisher.publish(&Diagnostics::new()).unwrap();
            }
        });
        for _ in 0..100 {
            assert_eq!(read(publisher.path()).schema_version, 1);
        }
    });
    assert!(read(publisher.path()).updated_at_ms > previous);
    assert_eq!(fs::read_dir(cache.path()).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn paths_are_private_and_symlinks_cannot_escape_the_root() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    fs::set_permissions(cache.path(), fs::Permissions::from_mode(0o755)).unwrap();
    let publisher = Publisher::in_directory(cache.path(), &uri(root.path()), "server").unwrap();
    symlink(outside.path(), root.path().join("escape")).unwrap();
    let data = [(uri(&root.path().join("escape/hidden.rs")), Vec::new())]
        .into_iter()
        .collect();
    publisher.publish(&data).unwrap();
    assert!(read(publisher.path()).diagnostics.is_empty());
    assert_eq!(
        fs::metadata(cache.path()).unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(
        fs::metadata(publisher.path()).unwrap().permissions().mode() & 0o777,
        0o600
    );
    symlink(cache.path(), root.path().join("cache-link")).unwrap();
    assert!(
        Publisher::in_directory(&root.path().join("cache-link"), &uri(root.path()), "server")
            .is_err()
    );
}
