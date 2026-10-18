//! Application identity lookups for shared atomic spawn admission.
codetether_spawn_policy::declare_identity_reservation!(
    pub(super), crate::tool::agent::store, crate::tool::agent::persistence,
);
#[cfg(test)]
#[path = "identity/scope_tests.rs"]
mod scope_tests;
