//! Existing-cache lifecycle tests, with no language server spawned.
#![cfg(unix)]
use super::*;
use codetether_diagnostics_bridge::{Diagnostics, Snapshot};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;

#[tokio::test(start_paused = true)]
async fn sharing_tracks_cache_clears_and_guard_drop() {
    let root = tempfile::tempdir().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let publisher = Publisher::in_directory(
        directory.path(),
        &format!("file://{}", root.path().display()),
        "existing-server",
    )
    .unwrap();
    let path = publisher.path().to_path_buf();
    let file = format!("file://{}/main.rs", root.path().display());
    let cache = Arc::new(RwLock::new(Diagnostics::new()));
    cache.write().await.insert(
        file.clone(),
        vec![lsp_types::Diagnostic::new_simple(
            Default::default(),
            "existing diagnostic".into(),
        )],
    );
    let alive = Arc::new(AtomicBool::new(true));
    let sharing = Sharing::from_cache(&cache, &alive, publisher);
    tokio::task::yield_now().await;
    let snapshot: Snapshot = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(snapshot.diagnostics[&file].len(), 1);
    cache.write().await.clear();
    tokio::time::advance(std::time::Duration::from_secs(2)).await;
    tokio::task::yield_now().await;
    let snapshot: Snapshot = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert!(snapshot.diagnostics.is_empty());
    drop(sharing);
    assert!(!path.exists());
    tokio::time::advance(std::time::Duration::from_secs(4)).await;
    tokio::task::yield_now().await;
    assert!(
        !path.exists(),
        "aborted pump must not resurrect the snapshot"
    );
}

#[tokio::test(start_paused = true)]
async fn sharing_removes_dead_reader_snapshot() {
    let root = tempfile::tempdir().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let publisher = Publisher::in_directory(
        directory.path(),
        &format!("file://{}", root.path().display()),
        "existing-server",
    )
    .unwrap();
    let path = publisher.path().to_path_buf();
    let cache = Arc::new(RwLock::new(Diagnostics::new()));
    let alive = Arc::new(AtomicBool::new(true));
    let _sharing = Sharing::from_cache(&cache, &alive, publisher);
    tokio::task::yield_now().await;
    assert!(path.exists());
    alive.store(false, Ordering::SeqCst);
    tokio::time::advance(std::time::Duration::from_secs(2)).await;
    tokio::task::yield_now().await;
    assert!(!path.exists());
}
