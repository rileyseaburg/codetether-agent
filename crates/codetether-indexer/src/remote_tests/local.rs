//! Local and graph-only indexing must not resolve remote credentials.

use super::*;
#[tokio::test]
async fn local_and_graph_only_builds_skip_remote_resolution() {
    let (dir, resolver, mut options) = fixture(10, 64);
    options.include_embeddings = false;
    let graph = build_index_with_resolver(dir.path(), &options, &resolver)
        .await
        .unwrap();
    assert_eq!(graph.stats.embedded_files, 0);
    options.include_embeddings = true;
    options.embedding_provider = "local".into();
    let local = build_index_with_resolver(dir.path(), &options, &resolver)
        .await
        .unwrap();
    assert_eq!(local.stats.embedded_files, 3);
    assert_eq!(resolver.calls.resolutions.load(Ordering::SeqCst), 0);
}
