//! Native cognition model ownership and blocking inference dispatch.
//!
//! [CandleThinker] loads model state when the candle feature is enabled;
//! disabled builds preserve the existing construction failure. [dispatch]
//! serializes inference with the original non-queueing mutex policy.
pub use codetether_cognition_core::{CandleDevicePreference, ThinkerConfig, ThinkerOutput};
#[cfg(feature = "candle")]
mod candle;
#[cfg(not(feature = "candle"))]
mod candle_disabled;
pub mod prefill;
#[cfg(feature = "candle")]
pub use candle::CandleThinker;
#[cfg(not(feature = "candle"))]
pub use candle_disabled::CandleThinker;
#[path = "candle_dispatch.rs"]
pub mod dispatch;

/// Explicit no-op router, independent of feature unification.
#[path = "routing_disabled.rs"]
pub mod disabled_routing;
/// Feature-selected FunctionGemma router or no-op adapter.
#[cfg(feature = "functiongemma")]
pub mod routing;
#[cfg(not(feature = "functiongemma"))]
pub use disabled_routing as routing;
