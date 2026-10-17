//! Fetching, lookup, and credential-aware catalog queries.
use crate::*;
use std::collections::HashMap;

/// Model catalog for looking up models
#[derive(Debug, Clone, Default)]
///
/// # Examples
/// ```
/// let catalog = codetether_model_catalog::ModelCatalog::new();
/// assert!(catalog.provider_ids().is_empty());
/// ```
pub struct ModelCatalog {
    providers: HashMap<String, ProviderInfo>,
}
mod all_providers;
mod available_providers;
mod available_providers_async;
mod check_provider_api_key_async;
mod fetch;
mod fetch_from;
mod find_model;
mod find_model_any;
mod get_available_model;
mod get_available_provider;
mod get_model;
mod get_provider;
mod models_for_provider;
mod new;
mod preload_available_providers;
mod provider_has_api_key;
mod provider_ids;
mod reasoning_models;
mod recommended_coding_models;
mod to_model_info;
mod tool_capable_models;
