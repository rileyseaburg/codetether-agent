//! Validation helpers for sub-agent spawning.
//!
//! This module enforces spawn-time depth and model eligibility policy.
//! Owner-scoped identity exclusion lives in `spawn::identity` so its claim can
//! remain active throughout asynchronous construction and persistence.
//!
//! # Examples
//!
//! ```ignore
//! let warning = validate_spawn_request(&request).await?;
//! ```

use super::policy;
use super::registry;
use super::spawn_request::SpawnRequest;
use crate::tool::ToolResult;

#[path = "spawn_depth.rs"]
mod spawn_depth;
#[path = "spawn_threads.rs"]
pub(in crate::tool::agent) mod spawn_threads;

/// Validates whether a requested sub-agent spawn is allowed.
///
/// Returns `Ok(Some(warning))` when the model is not free/subscription
/// eligible (the spawn proceeds with a warning), `Ok(None)` when fully
/// eligible, and `Err` for invalid depth or provider inspection failures.
///
/// # Examples
///
/// ```ignore
/// let warning = validate_spawn_request(&request).await?;
/// ```
pub(super) async fn validate_spawn_request(
    request: &SpawnRequest<'_>,
) -> Result<Option<String>, ToolResult> {
    spawn_depth::validate(request).await?;
    model_eligibility_warning(request.model).await
}

async fn model_eligibility_warning(model: &str) -> Result<Option<String>, ToolResult> {
    let registry = match registry::get_registry().await {
        Ok(registry) => registry,
        Err(error) => return Err(ToolResult::error(error.to_string())),
    };
    if policy::is_free_or_eligible(model, &registry).await {
        return Ok(None);
    }
    Ok(Some(ineligible_model_warning(model)))
}

fn ineligible_model_warning(model: &str) -> String {
    format!(
        "warning: '{model}' is not free/subscription-eligible; \
         this spawn may incur metered costs. Prefer ':free' models, \
         subscription providers, or OAuth providers."
    )
}

#[cfg(test)]
#[path = "spawn_validation_tests.rs"]
mod tests;
