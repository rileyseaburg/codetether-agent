//! Validation helpers for sub-agent spawning.
//!
//! This module enforces spawn-time depth and model eligibility policy.
//! Owner-scoped identity exclusion lives in `spawn::identity` so its claim can
//! remain active throughout asynchronous construction and persistence.
//!
//! # Examples
//!
//! ```ignore
//! let warning = validate_spawn_request(&request).await?;
//! ```

codetether_spawn_policy::declare_spawn_validation!(
    pub(super),
    crate::tool::agent::spawn_request::SpawnRequest<'_>,
    crate::tool::agent::store::lineage_for_session,
    crate::tool::agent::registry::get_registry,
);

#[cfg(test)]
#[path = "spawn_validation_tests.rs"]
mod tests;
