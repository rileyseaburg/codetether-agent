//! Local "thinking" models for edge inference.
//!
//! Supports OpenAI-compatible HTTP, the shared provider registry, Amazon
//! Bedrock, and optional in-process Candle inference as backends for fast
//! local reasoning completions.
//!
//! Start with [`ThinkerClient::new`] to create a client.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::cognition::{ThinkerBackend, ThinkerConfig};
//!
//! let cfg = ThinkerConfig::default();
//! assert_eq!(cfg.backend, ThinkerBackend::OpenAICompat);
//! ```
//!
//! # Architecture
//!
//! Contracts live in `backend`, `device_pref`, `config`, and `output`. Each
//! backend owns a module pair for request building and response decoding. The
//! Candle runtime compiles only with the `candle` feature; without it,
//! `candle_disabled` fails construction with a rebuild hint.

#[path = "thinker/backend.rs"]
mod backend;
#[path = "thinker/config.rs"]
mod config;
#[path = "thinker/device_pref.rs"]
mod device_pref;
#[path = "thinker/output.rs"]
mod output;

#[path = "thinker/client.rs"]
mod client;
#[path = "thinker/client_backend.rs"]
mod client_backend;
#[path = "thinker/client_build.rs"]
mod client_build;

#[path = "thinker/openai_backend.rs"]
mod openai_backend;
#[path = "thinker/openai_backend_request.rs"]
mod openai_backend_request;
#[path = "thinker/openai_backend_response.rs"]
mod openai_backend_response;
#[path = "thinker/openai_backend_send.rs"]
mod openai_backend_send;
#[path = "thinker/openai_backend_status.rs"]
mod openai_backend_status;
#[path = "thinker/openai_backend_trace.rs"]
mod openai_backend_trace;
#[path = "thinker/openai_wire/mod.rs"]
mod openai_wire;
#[path = "thinker/retry.rs"]
mod retry;

#[path = "thinker/bedrock_backend.rs"]
mod bedrock_backend;
#[path = "thinker/bedrock_request.rs"]
mod bedrock_request;
#[path = "thinker/bedrock_response.rs"]
mod bedrock_response;

#[path = "thinker/provider.rs"]
mod provider;
#[path = "thinker/provider_output.rs"]
mod provider_output;
#[path = "thinker/provider_request.rs"]
mod provider_request;

#[path = "thinker/candle_dispatch.rs"]
mod candle_dispatch;

#[cfg(not(feature = "candle"))]
#[path = "thinker/candle_disabled.rs"]
mod candle_disabled;

#[cfg(feature = "candle")]
#[path = "thinker/candle.rs"]
mod candle;

pub use backend::ThinkerBackend;
pub use client::ThinkerClient;
pub use config::ThinkerConfig;
pub use device_pref::CandleDevicePreference;
pub use output::ThinkerOutput;

#[cfg(feature = "candle")]
use candle::CandleThinker;
#[cfg(not(feature = "candle"))]
use candle_disabled::CandleThinker;

/// Candle runtime handle for crate-internal consumers such as the tool router.
#[cfg(feature = "candle")]
pub(super) use candle::CandleThinker as CandleRuntime;
