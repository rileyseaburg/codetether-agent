//! is local embedding provider for codebase indexing.

pub(in crate::indexer) fn is_local_embedding_provider(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "local" | "hash" | "hash-embedding" | "local/hash-embedding"
    )
}
