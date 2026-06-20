//! `local_cuda` provider for builds without CUDA support compiled in.
//!
//! The CUDA-backed implementation lives behind `#[cfg(feature = "candle-cuda")]`.
//! This module keeps the public API surface stable on non-CUDA builds; every
//! method returns a descriptive error so callers fail fast.

use super::*;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use futures::stream::BoxStream;

#[path = "local_cuda_nocuda/config.rs"]
mod config;
pub use config::LocalCudaConfig;

fn feature_error() -> anyhow::Error {
    anyhow!("local_cuda requires --features candle-cuda")
}

/// Provider type for builds without CUDA support compiled in.
///
/// All methods return errors; use `cfg(feature = "candle-cuda")` for the
/// real implementation.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::local_cuda::LocalCudaProvider;
/// assert!(!LocalCudaProvider::is_cuda_available());
/// ```
pub struct LocalCudaProvider;

impl LocalCudaProvider {
    pub fn new(_m: String) -> Result<Self> {
        Err(feature_error())
    }
    pub fn with_model(_m: String, _p: String) -> Result<Self> {
        Err(feature_error())
    }
    pub fn with_paths(
        _m: String,
        _p: String,
        _t: Option<String>,
        _a: Option<String>,
    ) -> Result<Self> {
        Err(feature_error())
    }
    pub fn is_cuda_available() -> bool {
        false
    }
    pub fn device_info() -> String {
        "CUDA unavailable".into()
    }
}

#[async_trait]
impl Provider for LocalCudaProvider {
    fn name(&self) -> &str {
        "local_cuda"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Err(feature_error())
    }
    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        Err(feature_error())
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        Err(feature_error())
    }
}
