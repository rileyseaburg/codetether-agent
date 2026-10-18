//! Automatic embeddings bound to the application's initialized providers.
pub use codetether_application_providers::embedding as provider_pick;
#[cfg(feature = "candle")]
pub use codetether_embedding_backends::auto::{bert_embedder, bert_forward, local_pick};
pub use codetether_embedding_backends::auto::{catalog, hf_download, resources};

pub use codetether_application_providers::embedding::auto_embedder;
