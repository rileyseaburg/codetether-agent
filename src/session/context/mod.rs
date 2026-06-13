//! Derive the per-step LLM context from an append-only chat history.
//!
//! See [`derive_context`] and [`derive_with_policy`] for entry points.

pub(crate) mod active_tail;
mod complete;
mod compress_step;
mod derive;
mod helpers;
mod incremental;
mod incremental_below_budget;
mod incremental_clamp;
mod incremental_coverage;
mod incremental_insert;
mod incremental_observability;
mod incremental_repair;
mod incremental_repair_drop;
mod incremental_repair_inject;
mod incremental_types;

#[cfg(test)]
mod incremental_coverage_tests;
#[cfg(test)]
mod incremental_repair_tests;
mod options;
mod policy;
mod policy_dispatch;
mod request;
mod reset;
mod reset_fallback;
mod reset_helpers;
mod reset_rebuild;
mod reset_summary;
mod reset_threshold;
#[cfg(test)]
mod reset_threshold_tests;
mod state_header;
mod state_header_pins;
#[cfg(test)]
mod state_header_pins_base_tests;
#[cfg(test)]
mod state_header_pins_tests;
#[cfg(test)]
mod state_header_tests;

#[cfg(test)]
mod active_tail_tests;

pub use self::complete::complete_with_context;
pub use self::derive::derive_context;
pub use self::helpers::DerivedContext;
pub use self::options::RequestOptions;
pub use self::policy::{derive_with_policy, effective_policy};
pub use self::request::build_request_with_context;
