//! Provider selection, model scoring and round-robin state.
//!
//! Supply a [ModelRegistry] to build a [RelayModelRotation]. Selection retains
//! configured provider precedence and the shared live Codex model catalog.
use codetether_provider_core::{ModelInfo, Provider};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
mod constants;
use constants::*;
mod state;
pub use state::RelayModelRotation;
mod registry;
pub use registry::ModelRegistry;
mod build_round_robin_model_rotation;
pub use build_round_robin_model_rotation::build_round_robin_model_rotation;
mod resolve_provider_for_model_autochat;
pub use resolve_provider_for_model_autochat::resolve_provider_for_model_autochat;
mod normalize_requested_model_ref;
use normalize_requested_model_ref::normalize_requested_model_ref;
mod add_unique_model_ref;
use add_unique_model_ref::add_unique_model_ref;
mod preferred_model_ref_for_provider;
use preferred_model_ref_for_provider::preferred_model_ref_for_provider;
mod choose_model_from_list;
use choose_model_from_list::choose_model_from_list;
mod choose_openrouter_model;
use choose_openrouter_model::choose_openrouter_model;
mod choose_copilot_model;
use choose_copilot_model::choose_copilot_model;
mod is_openrouter_free_model;
use is_openrouter_free_model::is_openrouter_free_model;
mod score_openrouter_model;
use score_openrouter_model::score_openrouter_model;
mod preferred_models_for_provider;
use preferred_models_for_provider::preferred_models_for_provider;
mod default_model_for_provider;
use default_model_for_provider::default_model_for_provider;
mod normalize_provider_name;
use normalize_provider_name::normalize_provider_name;
#[cfg(test)]
mod tests;
