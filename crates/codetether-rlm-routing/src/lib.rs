//! Application RLM routing context and shared provider/event adaptation.
//!
//! [RlmRouter] preserves routing and truncation entry points while converting
//! [AutoProcessContext] into the core RLM request. [Bus] implementations deliver
//! progress and completion events through the embedding application's bus.
mod bus;
mod context;
mod convert;
mod router;
pub use bus::Bus;
pub use context::{AutoProcessContext, ProcessProgress, RoutingContext, RoutingResult};
pub use convert::convert_ctx;
pub use router::RlmRouter;
