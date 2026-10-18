//! Local CUDA provider construction, configuration and inference.
//!
//! With the candle-cuda feature, supply a `Runtime` backend to
//! [LocalCudaProvider]. Without that feature, construction returns the existing
//! rebuild instruction and CUDA capability queries remain false.
#[cfg(feature = "candle-cuda")]
mod cuda;
#[cfg(feature = "candle-cuda")]
pub use cuda::{LocalCudaProvider, Runtime};
#[cfg(not(feature = "candle-cuda"))]
mod disabled;
#[cfg(feature = "candle-cuda")]
mod thinker;
pub use codetether_provider_core::local::LocalCudaConfig;
#[cfg(not(feature = "candle-cuda"))]
pub use disabled::LocalCudaProvider;

/// Boxed construction shared by enabled and disabled local providers.
pub mod construction;
