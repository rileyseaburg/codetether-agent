//! Retries preserve batch contents and count only successful usage.

use super::*;
#[tokio::test]
async fn remote_retry_preserves_batches_and_usage() {
    let (dir, resolver, options) = fixture(1, 64);
    let index = build_index_with_resolver(dir.path(), &options, &resolver)
        .await
        .unwrap();
    assert_eq!(resolver.calls.resolutions.load(Ordering::SeqCst), 1);
    assert_eq!(index.embedding_provider, "fixture-provider");
    assert_eq!(index.embedding_model, "resolved-model");
    assert_eq!(index.stats.embedded_files, 3);
    assert_eq!(index.stats.embedding_prompt_tokens, 22);
    assert_eq!(index.stats.embedding_total_tokens, 26);
    let requests = resolver.calls.requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    assert_eq!(requests[0].inputs, requests[1].inputs);
    assert_eq!(requests[2].inputs.len(), 1);
}
#[tokio::test]
async fn remote_retries_stop_at_the_configured_limit() {
    let (dir, resolver, options) = fixture(10, 64);
    let error = build_index_with_resolver(dir.path(), &options, &resolver)
        .await
        .unwrap_err();
    assert!(format!("{error:#}").contains("after 2 attempt(s)"));
    assert_eq!(resolver.calls.requests.lock().unwrap().len(), 2);
}
