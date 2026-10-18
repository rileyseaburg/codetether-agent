//! Native compatibility exports for shared RLM routing.
//!
//! Context conversion and router methods live in codetether-rlm-routing;
//! adapter_bus binds the application SessionBus to shared RLM event delivery.
pub mod adapter_bus;
pub mod adapter_convert;
pub mod adapter_provider;
pub mod context;
pub mod convert;
pub mod host_impl;
pub use codetether_rlm_routing::RlmRouter;
pub use context::{AutoProcessContext, ProcessProgress, RoutingContext, RoutingResult};
