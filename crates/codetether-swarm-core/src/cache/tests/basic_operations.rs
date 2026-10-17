use super::*;

#[tokio::test]
async fn test_cache_basic_operations() {
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

    let task = create_test_task("test task", "do something");
    let result = create_test_result(true);

    // Initially should be a miss
    assert!(cache.get(&task).await.is_none());
    assert_eq!(cache.stats().misses, 1);

    // Store the result
    cache.put(&task, &result).await.unwrap();

    // Now should be a hit
    let cached = cache.get(&task).await;
    assert!(cached.is_some());
    assert_eq!(cache.stats().hits, 1);
    assert_eq!(cached.unwrap().result, result.result);
}
