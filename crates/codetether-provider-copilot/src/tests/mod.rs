//! Existing endpoint and catalog regression tests.
use super::*;
use crate::catalog::filter::is_dated_model_variant;
mod enrich_with_pricing_sets_costs;
mod enterprise_base_url_uses_copilot_api_subdomain;
mod is_dated_model_variant_detects_date_suffix;
mod known_models_fallback_is_non_empty;
mod normalize_enterprise_domain_handles_scheme_and_trailing_slash;
