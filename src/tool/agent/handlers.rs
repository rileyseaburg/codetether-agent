//! Focused legacy action handler exports.

mod host {
    codetether_agent_lifecycle::declare_control_host!(
        pub struct ApplicationHost;,
        crate::session::Session,
        crate::tool::agent::persistence,
        crate::tool::agent::store,
        crate::tool::agent::collaboration_runtime::message_queue,
        crate::tool::agent::bus_publish::announce_done,
        crate::tool::agent::event_loop::live_trace::clear,
    );
}
#[path = "handlers/interrupt.rs"]
mod interrupt;
mod kill {
    codetether_agent_lifecycle::declare_kill_handler!(
        pub(in crate::tool::agent) fn handle; super::host::ApplicationHost
    );
}
#[path = "handlers/list.rs"]
mod list;

pub(super) use interrupt::handle as handle_interrupt;
pub(super) use kill::handle as handle_kill;
pub(super) use list::handle as handle_list;
