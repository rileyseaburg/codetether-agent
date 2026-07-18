use super::singleflight::LoadLocks;
use std::time::Duration;

#[tokio::test]
async fn unrelated_child_loads_do_not_block_each_other() {
    let locks = LoadLocks::default();
    let _first = locks.acquire("child-a").await;
    let second = tokio::time::timeout(Duration::from_secs(1), locks.acquire("child-b")).await;
    assert!(second.is_ok());
}
