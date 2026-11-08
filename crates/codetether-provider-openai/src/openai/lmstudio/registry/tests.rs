//! Backend aliases share admission while separate ports remain independent.
use super::*;
use std::time::Duration;
#[tokio::test]
async fn lmstudio_registry_shares_aliases_without_cross_backend_leaks() {
    let config = Config {
        output: 4096,
        waiting: 0,
        wait: Duration::from_secs(1),
        generation: Duration::from_secs(1),
    };
    let first = gate("http://LOCALHOST:39001/v1", config).unwrap();
    let alias = gate("http://localhost:39001/other", config).unwrap();
    let other = gate("http://localhost:39002/v1", config).unwrap();
    assert!(Arc::ptr_eq(&first, &alias));
    assert!(!Arc::ptr_eq(&first, &other));
    let lease = first.acquire().await.unwrap();
    assert!(alias.acquire().await.is_err());
    assert!(other.acquire().await.is_ok());
    drop(lease);
    assert!(alias.acquire().await.is_ok());
    assert!(
        gate(
            "http://localhost:39001/v1",
            Config {
                waiting: 2,
                ..config
            }
        )
        .is_err()
    );
}
