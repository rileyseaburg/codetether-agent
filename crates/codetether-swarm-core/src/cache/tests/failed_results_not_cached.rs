use super::*;

#[tokio::test]
async fn test_cache_failed_results_not_cached() {
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
    let failed_result = create_test_result(false);

    // Try to store failed result
    cache.put(&task, &failed_result).await.unwrap();

    // Should not be cached
    assert!(cache.get(&task).await.is_none());
}
