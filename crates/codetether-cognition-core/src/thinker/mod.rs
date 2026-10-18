//! Shared thinker contracts and configuration assembly.
//!
//! [ThinkerConfig] selects a [ThinkerBackend] and device preferences without
//! constructing clients or loading models. [ThinkerOutput] is backend-neutral.
mod backend;
pub mod candle_env;
mod config;
mod device_pref;
pub mod endpoint;
mod from_env;
mod output;
pub use backend::ThinkerBackend;
pub use config::ThinkerConfig;
pub use device_pref::CandleDevicePreference;
pub use from_env::thinker_config_from_env;
pub use output::ThinkerOutput;

/// Wire types and request/response conversion for thinker backends.
pub mod protocol;

/// OpenAI-compatible HTTP transport and retry lifecycle.
pub mod http;

/// Completion through a host-supplied signed Bedrock request.
pub mod bedrock;
/// Completion through an already resolved provider.
pub mod provider;

/// Backend-neutral thinker client ownership and dispatch.
pub mod client;
