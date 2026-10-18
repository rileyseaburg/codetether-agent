//! Background RLM summarisation for large tool outputs.

mod cache;
mod cache_read;
mod context;
mod disk;
mod gate;
mod job;
mod key;
mod render;
mod resolve;
mod route;
mod route_ctx;
mod status;
mod summary;
mod thread;

use codetether_provider_core::Message;

pub use resolver::{Resolution, Resolver};
pub use status::Notify;
mod resolver;

/// Replace pending background placeholders with ready cached summaries.
pub fn resolve_pending(messages: &mut [Message]) {
    resolve::messages(messages)
}

mod api_context;
mod api_route;
pub use api_context::context_summary;
pub use api_route::route_or_defer;
mod route_spawn;
