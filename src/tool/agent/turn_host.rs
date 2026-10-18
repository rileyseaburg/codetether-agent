//! Native effects selected for shared child-turn coordination.
pub(super) use super::event_loop::ApplicationHost;
codetether_child_turn::impl_child_turn!(
    crate::tool::agent::event_loop::ApplicationHost,
    crate::session::Session,
    crate::session::SessionEvent,
    crate::provider::ProviderRegistry;
    crate::tool::agent::store,
    crate::tool::agent::bus_publish,
    crate::tool::agent::collaboration_runtime::message_queue,
    crate::session::helper::steering::clear,
    crate::session::helper::steering::open,
    crate::tool::agent::bus_publish::announce_working,
    crate::tool::agent::collaboration_runtime::message_queue::finished;
    crate::tool::agent::helpers::get_registry,
    crate::tool::agent::spawn_run::start::begin,
    crate::tool::agent::message::start::begin,
    crate::tool::agent::message::remote_fallback::execute,
    codetether_agent_session::prepare_message,
    codetether_agent_session::ChildSession::ensure_bus,
);
