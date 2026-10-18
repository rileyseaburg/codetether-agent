//! Native thinker services, belief extraction and local-router selection.
//!
//! [ThinkerClient] shares the native provider registry. [beliefs] exposes
//! structured extraction, while [tool_router] selects the enabled local router
//! or its disabled adapter according to the application feature flags.
mod thinker;
pub use thinker::*;
pub mod beliefs;
#[doc(hidden)]
pub mod defaults;
pub mod text_util;
#[cfg(all(feature = "functiongemma", feature = "candle"))]
pub mod tool_router;
#[cfg(not(all(feature = "functiongemma", feature = "candle")))]
#[allow(dead_code)]
#[path = "tool_router_disabled.rs"]
pub mod tool_router;
