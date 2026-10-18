//! Parse a request alias without transport or credential access.

use super::*;

/// Decode an explicit Ultrafast alias or a catalog-supported Fast alias.
///
/// # Arguments
/// * `model` - Model ID after removing any reasoning-effort suffix.
/// # Returns
/// The upstream model ID and the requested service tier, if present.
/// # Examples
/// ```
/// use codetether_provider_core::codex::{parse_service_tier_model_alias, CodexServiceTier};
/// let (model, tier) = parse_service_tier_model_alias("gpt-6.1-sol-ultrafast");
/// assert_eq!(model, "gpt-6.1-sol");
/// assert_eq!(tier, Some(CodexServiceTier::Ultrafast));
/// ```
pub fn parse_service_tier_model_alias(model: &str) -> (String, Option<CodexServiceTier>) {
    // An explicit alias can come from another process's live picker catalog.
    // Decode it even when this process has no matching capability snapshot;
    // the backend remains responsible for checking service-tier entitlement.
    if let Some(base) = model.strip_suffix("-ultrafast") {
        return (base.to_string(), Some(CodexServiceTier::Ultrafast));
    }
    if let Some(base) = service_tier_catalog::parse_fast_alias(model) {
        return (base.to_string(), Some(CodexServiceTier::Priority));
    }
    (model.to_string(), None)
}

#[cfg(test)]
#[path = "parse_service_tier_model_alias_tests.rs"]
mod tests;
