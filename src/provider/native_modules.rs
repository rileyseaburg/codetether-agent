// Native-provider catalog and feature-gated implementations.
pub mod local_catalog;
#[cfg(feature = "candle-cuda")]
pub mod local_cuda;
#[cfg(not(feature = "candle-cuda"))]
#[allow(dead_code)]
#[path = "local_cuda_nocuda.rs"]
pub mod local_cuda;
