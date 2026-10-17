//! Retry PRD generation with available alternate models, then the primary model again.

use crate::{Registry, alternatives::get_model_alternatives, generate_prd_from_task};
use anyhow::Result;
use codetether_okr::Okr;
use codetether_provider_core::Provider;
use codetether_ralph_core::Prd;
use std::sync::Arc;
use tracing::{info, warn};
/// Generate a PRD using the primary model and configured alternative providers.
///
/// # Arguments
/// * `task` — Task to turn into a PRD.
/// * `okr` — Objective and expected key results.
/// * `primary_provider` — Provider used before and after alternative attempts.
/// * `primary_model` — Model selected by the caller.
/// * `registry` — Optional registry used to find alternatives in a fixed order.
///
/// # Returns
/// The first successfully generated PRD.
///
/// # Errors
/// Returns the final primary-model error if all attempts fail.
///
/// # Panics
/// The existing byte-based repair excerpt can panic on a UTF-8 boundary.
///
/// # Examples
/// ```no_run
/// async fn generate(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, okr: &codetether_okr::Okr)
///     -> anyhow::Result<codetether_ralph_core::Prd> {
///     codetether_go_prd::generate_prd_with_model_retry("Implement feature", okr, provider, "model", None).await
/// }
/// ```

/// Generate a PRD with automatic model retry on failure.
/// Tries the primary model first, then attempts alternative models if it fails.
/// This makes autonomous execution more robust against transient provider issues.
pub async fn generate_prd_with_model_retry(
    task: &str,
    okr: &Okr,
    primary_provider: Arc<dyn Provider>,
    primary_model: &str,
    registry: Option<&dyn Registry>,
) -> Result<Prd> {
    // Try primary model first
    match generate_prd_from_task(task, okr, primary_provider.as_ref(), primary_model).await {
        Ok(prd) => {
            info!(target:"codetether_agent::cli::go_ralph",model = %primary_model, "PRD generated with primary model");
            return Ok(prd);
        }
        Err(e) => {
            warn!(target:"codetether_agent::cli::go_ralph",model = %primary_model, error = %e, "Primary model failed, trying alternatives");
        }
    }

    // If registry provided, try alternative models
    if let Some(reg) = registry {
        let alternatives = get_model_alternatives(primary_model, reg);

        for (provider_name, alt_model) in alternatives {
            if let Some(provider) = reg.get(provider_name) {
                info!(target:"codetether_agent::cli::go_ralph",model = %alt_model, provider = %provider_name, "Trying alternative model");
                match generate_prd_from_task(task, okr, provider.as_ref(), &alt_model).await {
                    Ok(prd) => {
                        info!(target:"codetether_agent::cli::go_ralph",model = %alt_model, "PRD generated with alternative model");
                        return Ok(prd);
                    }
                    Err(e) => {
                        warn!(target:"codetether_agent::cli::go_ralph",model = %alt_model, error = %e, "Alternative model failed");
                        continue;
                    }
                }
            }
        }
    }

    // If we get here, all models failed - return original error
    generate_prd_from_task(task, okr, primary_provider.as_ref(), primary_model).await
}
