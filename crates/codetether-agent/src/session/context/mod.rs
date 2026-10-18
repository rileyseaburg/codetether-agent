//! Derive the per-step LLM context from an append-only chat history.
//!
//! See [`derive_context`] and [`derive_with_policy`] for entry points.

pub(crate) mod active_tail;
mod helpers;
mod incremental;
mod incremental_budget_config;
pub(crate) mod input_budget;

use codetether_context_policy::derivation::options;
mod policy;
mod request;
#[cfg(test)]
mod state_header;
#[cfg(test)]
mod state_header_pins;

#[cfg(test)]
include!("tests_manifest.rs");

pub use self::helpers::DerivedContext;
pub use self::options::RequestOptions;
pub use self::policy::{derive_with_policy, effective_policy};
pub use self::request::build_request_with_context;
pub use codetether_native_context::{complete_with_context, derive_context};

use super::Session;
codetether_session_schema::impl_session_context_views!(Session);
