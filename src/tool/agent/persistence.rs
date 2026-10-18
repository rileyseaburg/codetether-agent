//! Durable spawned-agent manifests and recovery orchestration.

codetether_agent_storage::declare_application_storage!(
    pub(in crate::tool::agent);
    crate::session::Session,
    crate::tool::agent::store,
    crate::tool::agent::collaboration_runtime::message_queue,
);

#[cfg(test)]
#[path = "persistence/close_receipt_tests.rs"]
mod close_receipt_tests;
#[cfg(test)]
#[path = "persistence/lifecycle_tests.rs"]
mod lifecycle_tests;
#[cfg(test)]
#[path = "persistence/migration_tests.rs"]
mod migration_tests;
#[cfg(test)]
#[path = "persistence/restart_tests.rs"]
mod restart_tests;
#[cfg(test)]
#[path = "persistence/restart_test_support.rs"]
pub(in crate::tool::agent) mod test_support;
