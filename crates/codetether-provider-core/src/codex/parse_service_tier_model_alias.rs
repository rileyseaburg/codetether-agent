//! Parse a request alias without transport or credential access.

use super::*;

pub fn parse_service_tier_model_alias(model: &str) -> (String, Option<CodexServiceTier>) {
    if let Some(base) = model
        .strip_suffix("-ultrafast")
        .filter(|base| service_tier_catalog::supports_ultrafast(base))
    {
        return (base.to_string(), Some(CodexServiceTier::Ultrafast));
    }
    if let Some(base) = service_tier_catalog::parse_fast_alias(model) {
        return (base.to_string(), Some(CodexServiceTier::Priority));
    }
    (model.to_string(), None)
}
