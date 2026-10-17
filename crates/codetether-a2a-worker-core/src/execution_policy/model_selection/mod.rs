//! Select an available provider and its effective model identifier.
//!
//! Availability errors and explicit selections take precedence over defaults.
use super::{choose_provider_for_tier, default_model_for_provider};
use anyhow::Result;
mod availability;
mod parse;
mod select;
pub use availability::ensure_providers;
use parse::parse_model;
pub use select::select;
/// Provider and model chosen for a worker execution.

pub struct ModelSelection {
    pub provider: String,
    pub model: String,
}
