//! Compatibility exports for the shared Bonsai provider and native model.
#[cfg(feature = "candle")]
pub(crate) use codetether_provider_bonsai::native;
pub use codetether_provider_bonsai::{BonsaiConfig, BonsaiProvider, GenerationTiming, MODEL};
