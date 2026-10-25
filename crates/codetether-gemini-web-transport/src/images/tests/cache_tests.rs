use crate::images::{cache::Cache, rotation::Rotation, rotation_fresh::fresh};

#[test]
fn private_cache_roundtrips_and_separates_source_exports() {
    let root = tempfile::tempdir().unwrap();
    let directory = root.path().join("cache");
    let cache = Cache::new(directory.clone(), "source-cookie-secret");
    assert!(!cache.key.contains("secret"));
    let cookie = Rotation {
        name: "__Secure-1PSIDTS".into(),
        value: "synthetic".into(),
        expires: 9999,
    };
    cache.save(100, &[cookie]).unwrap();
    let loaded = cache.load().unwrap();
    assert_eq!(loaded.cookies[0].value, "synthetic");
    assert!(
        Cache::new(directory.clone(), "different-source")
            .load()
            .is_none()
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let file = directory.join(format!("{}.json", cache.key));
        assert_eq!(
            std::fs::metadata(&file).unwrap().permissions().mode() & 0o777,
            0o600
        );
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(cache.load().is_none());
    }
}

#[test]
fn renewal_throttle_rejects_future_timestamps() {
    assert!(fresh(100, 159));
    assert!(!fresh(100, 160));
    assert!(!fresh(100, 99));
    assert!(!fresh(0, 1));
}

#[test]
fn malformed_cache_is_not_trusted() {
    let root = tempfile::tempdir().unwrap();
    let cache = Cache::new(root.path().join("cache"), "source");
    cache.save(100, &[]).unwrap();
    let file = cache.directory.join(format!("{}.json", cache.key));
    std::fs::write(file, "{not valid json}").unwrap();
    assert!(cache.load().is_none());
}
