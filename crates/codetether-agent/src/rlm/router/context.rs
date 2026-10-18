//! Native specialization of the shared RLM application context.
pub use codetether_rlm_routing::{ProcessProgress, RoutingContext, RoutingResult};
/// Routing inputs with the application's concrete session event bus.
pub type AutoProcessContext<'a> =
    codetether_rlm_routing::AutoProcessContext<'a, crate::session::SessionBus>;
