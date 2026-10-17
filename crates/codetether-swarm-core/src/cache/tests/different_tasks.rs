use super::*;

#[tokio::test]
async fn test_cache_different_tasks() {
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

    let task1 = create_test_task("task 1", "do something");
    let task2 = create_test_task("task 2", "do something else");
    let result = create_test_result(true);

    // Store only task1
    cache.put(&task1, &result).await.unwrap();

    // task1 should hit, task2 should miss
    assert!(cache.get(&task1).await.is_some());
    assert!(cache.get(&task2).await.is_none());
}
