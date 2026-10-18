//! Handle the "message" action.
#[path = "remote/mod.rs"]
pub(super) mod remote;

codetether_child_turn::declare_message_entrypoints!(
    crate::tool::agent::turn_host::ApplicationHost,
    crate::session::Session,
    crate::session::SessionEvent,
    crate::tool::agent::message::remote::message_or_missing,
);
