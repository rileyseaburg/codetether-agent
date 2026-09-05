//! Deterministic publication timing coverage without a language-server process.

use super::*;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test(start_paused = true)]
async fn unrelated_publications_do_not_complete_a_cold_document_request() {
    let cache = Arc::new(RwLock::new(HashMap::from([("other".into(), vec![])])));
    let published = Arc::clone(&cache);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        published
            .write()
            .await
            .insert("target".into(), vec![Diagnostic::default()]);
    });
    let start = tokio::time::Instant::now();
    let diagnostics = wait(
        || async { cache.read().await.clone() },
        "target",
        Duration::from_secs(3),
    )
    .await
    .unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert!(start.elapsed() >= Duration::from_secs(2));
}

#[tokio::test(start_paused = true)]
async fn an_explicit_empty_publication_is_a_valid_result() {
    let result = wait(
        || async { HashMap::from([("target".into(), vec![])]) },
        "target",
        Duration::from_secs(1),
    )
    .await
    .unwrap();
    assert!(result.is_empty());
}

#[tokio::test(start_paused = true)]
async fn missing_publication_is_an_error_not_clean_diagnostics() {
    let error = wait(
        || async { HashMap::new() },
        "target",
        Duration::from_secs(1),
    )
    .await
    .unwrap_err();
    assert!(error.to_string().contains("no publication"));
    assert!(error.to_string().contains("target"));
}
