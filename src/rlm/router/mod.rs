//! Native compatibility exports for shared RLM routing.
//!
//! Context conversion and router methods live in codetether-rlm-routing;
//! adapter_bus binds the application SessionBus to shared RLM event delivery.
pub use codetether_rlm::provider_adapter as adapter_convert;
pub use codetether_rlm::provider_adapter as adapter_provider;
pub use codetether_rlm_routing::session_bus as adapter_bus;
pub mod context;
pub use codetether_rlm::router as host_impl;
pub use codetether_rlm_routing as convert;
pub use codetether_rlm_routing::RlmRouter;
pub use context::{AutoProcessContext, ProcessProgress, RoutingContext, RoutingResult};
