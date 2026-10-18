//! Native registry specialization of shared worker request context.
pub(super) type Context<'a> =
    codetether_a2a_worker_core::session_protocol::context::Context<'a, crate::tool::ToolRegistry>;
