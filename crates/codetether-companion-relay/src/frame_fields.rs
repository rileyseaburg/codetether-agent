//! Optional frame fields: trigger and opaque owner request ID.
use crate::{ApiError, paths::uuid_like};
use codetether_companion_protocol::CaptureTrigger;
use serde_json::Value;

/// Parse `trigger` and `request_id`; explicit nulls and unknowns are rejected.
pub(crate) fn parse(value: &Value) -> Result<(Option<CaptureTrigger>, Option<String>), ApiError> {
    let trigger = match value.get("trigger") {
        None => None,
        Some(raw) => Some(
            serde_json::from_value::<CaptureTrigger>(raw.clone())
                .map_err(|_| ApiError::new(400, "Invalid capture trigger"))?,
        ),
    };
    let request_id = match value.get("request_id") {
        None => None,
        Some(Value::String(id)) if uuid_like(id) => Some(id.clone()),
        Some(_) => return Err(ApiError::new(400, "Invalid capture request")),
    };
    Ok((trigger, request_id))
}
