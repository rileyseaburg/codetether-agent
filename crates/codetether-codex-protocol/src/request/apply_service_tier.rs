//! apply service tier.
use super::*;
impl CodexProtocol {
    /// Insert the requested service tier; leave the payload unchanged when absent.
    pub fn apply_service_tier(payload: &mut Value, service_tier: Option<CodexServiceTier>) {
        if let Some(service_tier) = service_tier {
            payload["service_tier"] = json!(service_tier.as_str());
        }
    }
}
