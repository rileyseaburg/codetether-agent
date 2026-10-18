//! CUDA device ownership, model configuration and provider execution.
//!
//! [LocalCudaProvider] loads its [Runtime] on first completion. The application
//! supplies backend construction; this crate owns all provider policy.
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use candle_core::Device;
use codetether_cognition_core::{CandleDevicePreference, ThinkerBackend, ThinkerConfig};
use codetether_provider_core::local::env::{
    first_env, parse_env_f32, parse_env_u64, parse_env_usize,
};
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, Usage,
};
use futures::stream::BoxStream;
use std::path::Path;
use tokio::sync::OnceCell;
mod catalog;
mod config;
mod construct;
mod host;
mod paths;
mod prompt;
mod provider;
mod runtime;
pub use host::Runtime;

/// Local CUDA Provider using Candle bindings
///
/// This provider uses pure-Rust ML execution directly on NVIDIA hardware
/// without needing C++ interop.
///
/// # Examples
/// ```no_run
/// # fn create<R: codetether_provider_local_cuda::Runtime>() -> anyhow::Result<()> {
/// let provider = codetether_provider_local_cuda::LocalCudaProvider::<R>::new("local-model".into())?;
/// # let _ = provider;
/// # Ok(())
/// # }
/// ```
pub struct LocalCudaProvider<R: Runtime> {
    model_name: String,
    _device: Device,
    /// Model cache - in production this would hold the loaded model
    model_cache: Option<ModelCache>,
    runtime: OnceCell<R>,
}

struct ModelCache {
    model_path: String,
    tokenizer_path: Option<String>,
    architecture: Option<String>,
}
#[cfg(test)]
mod tests;
