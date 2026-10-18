//! Native request entry point and shared host bindings.
use crate::session::Session;
codetether_context_request::impl_session_context!(
    Session,
    super::policy::effective_policy,
    crate::session::index_produce::proactive::prepare,
    super::policy::derive_with_policy,
    crate::session::index::recall::prefetch::message
);

pub use codetether_native_context::build_request_with_context;
