//! Validation helpers for sub-agent spawning.
//!
//! This module enforces spawn-time policy checks such as model
//! eligibility and name uniqueness. It intentionally allows the caller
//! and child agent to use the same model.
//!
//! # Examples
//!
//! ```ignore
//! validate_spawn_request(&request).await?;
//! ```

use super::policy;
use super::registry;
use super::spawn_request::SpawnRequest;
use super::store;
use crate::tool::ToolResult;

/// Validates whether a requested sub-agent spawn is allowed.
///
/// The validator enforces model eligibility and prevents duplicate agent
/// names before the session factory creates any stateful resources.
///
/// # Examples
///
/// ```ignore
/// validate_spawn_request(&request).await?;
/// ```
pub(super) async fn validate_spawn_request(request: &SpawnRequest<'_>) -> Result<(), ToolResult> {
    validate_model(request.model).await?;
    validate_name_is_available(request.name)
}

async fn validate_model(model: &str) -> Result<(), ToolResult> {
    let registry = match registry::get_registry().await {
        Ok(registry) => registry,
        Err(error) => return Err(ToolResult::error(error.to_string())),
    };
    if policy::is_free_or_eligible(model, &registry).await {
        return Ok(());
    }
    Err(ToolResult::error(ineligible_model_message(model)))
}

fn validate_name_is_available(name: &str) -> Result<(), ToolResult> {
    if !store::contains(name) {
        return Ok(());
    }
    Err(ToolResult::error(format!(
        "Agent @{name} exists. Use kill first."
    )))
}

fn ineligible_model_message(model: &str) -> String {
    format!(
        "Spawn blocked: '{model}' is not free/subscription-eligible. \
         Use ':free' models, subscription providers, or OAuth providers."
    )
}
