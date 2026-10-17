//! embedding responsibilities for codebase indexing.

use super::*;
mod embedding_backend;
pub(super) use embedding_backend::EmbeddingBackend;
mod remote_embedding_engine;
pub(super) use remote_embedding_engine::RemoteEmbeddingEngine;
mod resolve_embedding_backend;
pub(super) use resolve_embedding_backend::resolve_embedding_backend;
mod is_local_embedding_provider;
pub(super) use is_local_embedding_provider::is_local_embedding_provider;
mod build_model_selector;
pub(super) use build_model_selector::build_model_selector;
mod retry_delay;
pub(super) use retry_delay::retry_delay;
mod is_retryable_embedding_error;
pub(super) use is_retryable_embedding_error::is_retryable_embedding_error;
mod approximate_token_count;
pub(super) use approximate_token_count::approximate_token_count;
mod build_embedding_input;
pub(super) use build_embedding_input::build_embedding_input;
mod safe_char_prefix;
pub(super) use safe_char_prefix::safe_char_prefix;
mod local_embedding_engine;
pub(super) use local_embedding_engine::LocalEmbeddingEngine;
mod tokenize_for_embedding;
pub(super) use tokenize_for_embedding::tokenize_for_embedding;
mod l2_normalize;
pub(super) use l2_normalize::l2_normalize;
