//! Native model-resolution and progress-event bindings for shared workers.
mod context;
mod host;
mod route;
pub(super) use codetether_rlm_background::resolve_pending;
pub(crate) use context::context_summary;
pub(crate) use route::route_or_defer;
pub(crate) type Notify = std::sync::Arc<dyn Fn(crate::session::SessionEvent) + Send + Sync>;

pub(crate) use host::resolve as resolve_model;
