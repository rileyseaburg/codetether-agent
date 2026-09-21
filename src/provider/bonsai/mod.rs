//! Dedicated native Bonsai provider. No agent loop, RLM recursion, HTTP backend or subprocess inference.
//! Configure checkpoint paths with [`BonsaiConfig`] and construct [`BonsaiProvider`].
mod completion;
mod config;
#[cfg(feature = "candle-cuda")]
mod decode;
#[cfg(feature = "candle-cuda")]
mod generate;
mod instance;
#[cfg(feature = "candle")]
pub(crate) mod native;
mod protocol;
mod provider_impl;
pub(crate) mod registration;
mod request;
#[cfg(feature = "candle-cuda")]
mod runtime;
#[cfg(feature = "candle-cuda")]
mod sampler;
mod stop;
mod stream;
mod timing;
#[cfg(feature = "candle-cuda")]
mod worker;
pub use config::BonsaiConfig;
pub use instance::BonsaiProvider;
pub use timing::GenerationTiming;
pub const MODEL: &str = "ternary-bonsai-2-27b-pq2";
