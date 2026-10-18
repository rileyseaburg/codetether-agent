//! Canonical child-thread registry keyed by durable session ID.

use crate::session::Session;
use codetether_agent_registry::Registry;

lazy_static::lazy_static! {
    static ref AGENT_STORE: Registry<Session> = Registry::default();
}

codetether_agent_registry::declare_application_registry!(
    pub(super),
    Session, AGENT_STORE,
    crate::tool::agent::residency::touch,
    crate::tool::agent::residency::forget,
    crate::tool::agent::collaboration_runtime::agent_tree::resolve,
);

#[cfg(test)]
#[path = "store_identity_tests.rs"]
mod identity_tests;
#[cfg(test)]
#[path = "store_scope_tests.rs"]
mod scope_tests;
