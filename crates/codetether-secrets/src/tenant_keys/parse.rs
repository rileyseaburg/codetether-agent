//! Parse optional task claims without constructing application registries.
use super::*;
use anyhow::Result;
impl TenantProviderKeyPayload {
    /// Parse a claim, ignoring null values and empty provider maps.
    ///
    /// # Arguments
    /// * `value` - Task claim JSON to decode.
    /// # Returns
    /// A populated payload or None when no provider keys were supplied.
    /// # Errors
    /// Returns a JSON decoding error for an incompatible claim shape.
    pub fn from_value(value: &serde_json::Value) -> Result<Option<Self>> {
        if value.is_null() {
            return Ok(None);
        }
        let payload: Self = serde_json::from_value(value.clone())?;
        if payload.providers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(payload))
        }
    }
}
