//! Shared overflow recovery with application warning logs.
mod derive_plan;
pub(crate) use codetether_context_policy::request::recovery::{Plan, recovery};
pub(crate) use derive_plan::resolve as derivation;
