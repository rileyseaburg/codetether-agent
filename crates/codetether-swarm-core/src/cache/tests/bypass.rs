use super::*;

#[tokio::test]
async fn test_cache_bypass() {
    let temp_dir = tempdir().unwrap();
    let config = CacheConfig {
        enabled: true,
        ttl_secs: 3600,
        max_entries: 100,
        max_size_mb: 10,
        cache_dir: Some(temp_dir.path().to_path_buf()),
        bypass: true, // Bypass enabled
    };

    let mut cache = SwarmCache::new(config, || panic!("explicit cache directory expected"))
        .await
        .unwrap();

    let task = create_test_task("test", "instruction");
    let result = create_test_result(true);

    // Store the result
    cache.put(&task, &result).await.unwrap();

    // Should still be a miss due to bypass
    assert!(cache.get(&task).await.is_none());
}
