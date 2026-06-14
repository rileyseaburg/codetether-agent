//! Validation helpers for sub-agent spawning.
//!
//! This module enforces spawn-time policy checks such as model
//! eligibility and name uniqueness. Model eligibility is advisory
//! (a warning), while duplicate names remain a hard error.
//!
//! # Examples
//!
//! ```ignore
//! let warning = validate_spawn_request(&request).await?;
//! ```

use super::policy;
use super::registry;
use super::spawn_request::SpawnRequest;
use super::store;
use crate::tool::ToolResult;

/// Validates whether a requested sub-agent spawn is allowed.
///
/// Returns `Ok(Some(warning))` when the model is not free/subscription
/// eligible (the spawn proceeds with a warning), `Ok(None)` when fully
/// eligible, and `Err` for hard failures such as duplicate agent names.
///
/// # Examples
///
/// ```ignore
/// let warning = validate_spawn_request(&request).await?;
/// ```
pub(super) async fn validate_spawn_request(
    request: &SpawnRequest<'_>,
) -> Result<Option<String>, ToolResult> {
    validate_name_is_available(request.name)?;
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

fn validate_name_is_available(name: &str) -> Result<(), ToolResult> {
    if !store::contains(name) {
        return Ok(());
    }
    Err(ToolResult::error(format!(
        "Agent @{name} exists. Use kill first."
    )))
}

fn ineligible_model_warning(model: &str) -> String {
    format!(
        "warning: '{model}' is not free/subscription-eligible; \
         this spawn may incur metered costs. Prefer ':free' models, \
         subscription providers, or OAuth providers."
    )
}
