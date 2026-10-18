//! Expiry and eviction must finish deleting persistent cache entries.
use super::*;

#[tokio::test]
async fn expired_and_evicted_entries_are_removed_from_disk() {
    let directory = tempdir().unwrap();
    let config = CacheConfig {
        enabled: true,
        ttl_secs: 60,
        max_entries: 1,
        max_size_mb: 10,
        cache_dir: Some(directory.path().to_path_buf()),
        bypass: false,
    };
    let mut cache = SwarmCache::new(config, || unreachable!()).await.unwrap();
    let first = create_test_task("first", "inspect first file");
    let second = create_test_task("second", "inspect second file");
    let key = SwarmCache::generate_key(&first);
    let path = directory.path().join(format!("{key}.json"));
    let result = create_test_result(true);

    cache.put(&first, &result).await.unwrap();
    assert!(path.exists());
    cache.index.get_mut(&key).unwrap().created_at = SystemTime::UNIX_EPOCH;
    assert!(cache.get(&first).await.is_none());
    assert!(!path.exists());

    cache.put(&first, &result).await.unwrap();
    cache.put(&second, &result).await.unwrap();
    assert!(!path.exists());
    assert!(cache.get(&first).await.is_none());
    assert!(cache.get(&second).await.is_some());
}
