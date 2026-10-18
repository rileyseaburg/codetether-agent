//! Derive the per-step LLM context from an append-only chat history.
//!
//! See [`derive_context`] and [`derive_with_policy`] for entry points.

pub(crate) mod active_tail;
mod complete;
mod derive;
mod helpers;
mod incremental;
mod incremental_budget_config;
use codetether_context_policy::projection::incremental_coverage;
use codetether_context_policy::projection::incremental_insert;
use codetether_context_policy::projection::incremental_repair;
use codetether_context_policy::projection::incremental_repair_drop;
use codetether_context_policy::projection::incremental_repair_inject;
use codetether_context_policy::projection::incremental_types;
use codetether_incremental_derivation as incremental_clamp;
pub(crate) mod input_budget;

mod dispatch;
use codetether_context_policy::derivation::options;
mod policy;
mod request;
pub(crate) use codetether_context_request as request_guard;
mod reset;
use codetether_context_policy::derivation::reset::markers as reset_helpers;
mod state_header;
mod state_header_pins;

#[cfg(test)]
include!("tests_manifest.rs");

pub use self::complete::complete_with_context;
pub use self::derive::derive_context;
pub use self::helpers::DerivedContext;
pub use self::options::RequestOptions;
pub use self::policy::{derive_with_policy, effective_policy};
pub use self::request::build_request_with_context;

use super::Session;
codetether_session_schema::impl_session_context_views!(Session);
