//! Translate command-line options without performing indexing.

use super::*;

pub(super) fn options(args: &IndexArgs) -> BuildOptions {
    let defaults = BuildOptions::default();
    BuildOptions {
        include_hidden: args.include_hidden,
        include_embeddings: true,
        max_file_size_bytes: args.max_file_size_kib.saturating_mul(1024),
        embedding_provider: if args.embedding_provider.trim().is_empty() {
            defaults.embedding_provider
        } else {
            args.embedding_provider.clone()
        },
        embedding_model: if args.embedding_model.trim().is_empty() {
            defaults.embedding_model
        } else {
            args.embedding_model.clone()
        },
        embedding_dimensions: args.embedding_dimensions.max(64),
        embedding_batch_size: args.embedding_batch_size.max(1),
        embedding_input_chars: args.embedding_input_chars.max(256),
        embedding_max_retries: args.embedding_max_retries,
        embedding_retry_initial_ms: args.embedding_retry_initial_ms.max(1),
        embedding_retry_max_ms: args
            .embedding_retry_max_ms
            .max(args.embedding_retry_initial_ms.max(1)),
    }
}
