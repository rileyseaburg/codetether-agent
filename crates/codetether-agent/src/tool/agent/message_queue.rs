//! Application ownership and execution for the native durable mailbox.
codetether_native_agent::declare_application_mailbox!(
    pub(in crate::tool::agent);
    crate::tool::agent::store,
    crate::tool::agent::message::run::execute,
);
