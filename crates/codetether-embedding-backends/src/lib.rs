//! Local and remote embedding backends with automatic host selection.
//!
//! [auto] detects resources and tries a fitting local BERT model before known
//! cloud providers. [ProviderEmbedder] supports remote asynchronous embeddings
//! and the existing hashing fallback for synchronous calls.
/// Automatic resource-aware backend selection and local model loading.
pub mod auto;
/// Remote provider embedding with synchronous local fallback.
pub mod provider;
pub use provider::ProviderEmbedder;
