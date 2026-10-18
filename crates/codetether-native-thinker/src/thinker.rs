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
//! use codetether_native_thinker::{ThinkerBackend, ThinkerConfig};
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

use codetether_cognition_core::thinker as backend;
use codetether_cognition_core::thinker as config;
use codetether_cognition_core::thinker as device_pref;
use codetether_cognition_core::thinker as output;

#[path = "thinker/client.rs"]
mod client;

use codetether_cognition_native::dispatch as candle_dispatch;

#[cfg(not(feature = "candle"))]
use codetether_cognition_native as candle_disabled;

#[cfg(feature = "candle")]
use codetether_cognition_native as candle;

pub use backend::ThinkerBackend;
#[doc(hidden)]
pub use client::Services;
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
#[doc(hidden)]
pub use candle::CandleThinker as CandleRuntime;
