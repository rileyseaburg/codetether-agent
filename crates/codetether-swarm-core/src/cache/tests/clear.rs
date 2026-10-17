use super::*;

#[tokio::test]
async fn test_cache_clear() {
    let temp_dir = tempdir().unwrap();
    let config = CacheConfig {
        enabled: true,
        ttl_secs: 3600,
        max_entries: 100,
        max_size_mb: 10,
        cache_dir: Some(temp_dir.path().to_path_buf()),
        bypass: false,
    };

    let mut cache = SwarmCache::new(config, || panic!("explicit cache directory expected"))
        .await
        .unwrap();

    let task = create_test_task("test", "instruction");
    let result = create_test_result(true);

    cache.put(&task, &result).await.unwrap();
    assert!(cache.get(&task).await.is_some());

    cache.clear().await.unwrap();
    assert!(cache.get(&task).await.is_none());
    assert_eq!(cache.stats().current_entries, 0);
}
