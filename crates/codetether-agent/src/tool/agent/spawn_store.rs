//! Application services for shared durable child registration.
codetether_agent_spawn::declare_application_registration!(
    pub(super),
    crate::session::Session,
    crate::tool::agent::store,
    crate::tool::agent::persistence,
);
