//! Bind PRD generation retries to the application provider registry.
use crate::{
    okr::Okr,
    provider::{Provider, ProviderRegistry},
    ralph::Prd,
};
use anyhow::Result;
use std::sync::Arc;
/// Generate a PRD with the application's configured provider registry.
///
/// # Arguments
/// * `task` — Task description.
/// * `okr` — Objective whose key results inform the PRD.
/// * `primary_provider` — Provider used for the first and final attempts.
/// * `primary_model` — Caller-selected model name.
/// * `registry` — Optional configured registry for alternative model attempts.
///
/// # Returns
/// The first successfully generated PRD.
///
/// # Errors
/// Returns the final generation error when all attempts fail.
///
/// # Panics
/// Preserves the generator's byte-based repair excerpt behavior.
///
/// # Examples
/// See [codetether_go_prd::generate_prd_with_model_retry] for a generation example.
pub async fn generate_prd_with_model_retry(
    task: &str,
    okr: &Okr,
    primary_provider: Arc<dyn Provider>,
    primary_model: &str,
    registry: Option<&ProviderRegistry>,
) -> Result<Prd> {
    codetether_go_prd::generate_prd_with_model_retry(
        task,
        okr,
        primary_provider,
        primary_model,
        registry.map(|r| r as &dyn codetether_go_prd::Registry),
    )
    .await
}
