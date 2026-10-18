//! Default request temperature after model capability checks.
use crate::{prefers_temperature_one, temperature::temperature_is_deprecated};

/// Choose the request temperature permitted by a model's parameter contract.
/// # Arguments
/// `model` is the possibly provider-qualified model identifier.
/// # Returns
/// None for models without temperature support, 1.0 when required, otherwise 0.7.
/// # Examples
/// ```
/// assert_eq!(codetether_provider_policy::temperature_for("bedrock/us.openai.gpt-6-astra"), None);
/// ```
pub fn temperature_for(model: &str) -> Option<f32> {
    if temperature_is_deprecated(model) {
        None
    } else if prefers_temperature_one(model) {
        Some(1.0)
    } else {
        Some(0.7)
    }
}
