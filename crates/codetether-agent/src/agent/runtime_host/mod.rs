//! Application-local marker selecting native agent session services.
/// Session operations bound to the application's existing runtime policies.
pub struct SessionServices;
codetether_native_agent::impl_session_services!(
    SessionServices,
    crate::session::Session,
    crate::session::helper::runtime::prior_context_tool_available_for_session,
    crate::session::helper::runtime::block_serialized_prior_context_for_session,
    crate::session::helper::runtime::enrich_tool_input_for_session,
);
/// Native registry and context services with application session bookkeeping.
pub type Native =
    codetether_native_agent::Host<SessionServices, crate::tool::registry_lifecycle::Native>;
