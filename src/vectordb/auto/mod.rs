//! Automatic embeddings bound to the application's initialized providers.
#[cfg(feature = "candle")]
pub use codetether_embedding_backends::auto::{bert_embedder, bert_forward, local_pick};
pub use codetether_embedding_backends::auto::{catalog, hf_download, resources};
pub mod provider_pick;

pub use codetether_application_providers::embedding::auto_embedder;
