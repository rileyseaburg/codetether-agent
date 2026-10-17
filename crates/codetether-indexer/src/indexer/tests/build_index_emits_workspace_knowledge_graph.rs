//! Indexer behavior regression.

use super::*;

#[tokio::test]
async fn build_index_emits_workspace_knowledge_graph() {
    let temp = tempdir().expect("tempdir");
    std::fs::write(temp.path().join("types.rs"), "pub struct Session;\n").expect("write");
    std::fs::write(
        temp.path().join("main.rs"),
        "use crate::types::Session;\nfn run() {}\n",
    )
    .expect("write");

    let index = build_index(
        temp.path(),
        &BuildOptions {
            include_embeddings: false,
            ..BuildOptions::default()
        },
    )
    .await
    .expect("index should build");

    assert_eq!(index.embedding_provider, DISABLED_EMBEDDING_PROVIDER);
    assert!(
        index
            .knowledge_graph
            .nodes
            .iter()
            .any(|node| node.kind == "symbol" && node.label == "Session")
    );
    assert!(
        index
            .knowledge_graph
            .edges
            .iter()
            .any(|edge| edge.kind == "imports_symbol" && edge.target.contains("Session"))
    );
}
