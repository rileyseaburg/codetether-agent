//! Read-only migration and Unix permission assertions for a valid fixture.
use crate::images::{cookies::Jar, node_cache};
use std::path::Path;

pub(super) fn verify(directory: &Path, key: &str, jar: &Jar) {
    let path = directory.join(format!("{key}.json"));
    let before = std::fs::read(&path).unwrap();
    assert!(node_cache::load(directory, jar).is_some());
    assert_eq!(before, std::fs::read(&path).unwrap());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(node_cache::load(directory, jar).is_none());
    }
}
