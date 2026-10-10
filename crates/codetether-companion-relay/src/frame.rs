//! Frame upload validation: size, base64, JPEG bounds, and freshness.
use crate::{ApiError, frame_fields, jpeg};
use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{DateTime, SecondsFormat};
use codetether_companion_protocol::Capture;
use serde_json::Value;

const SHAPE: &str = "Expected a bounded JPEG screenshot and capture time";
const IMAGE: &str = "Screenshot must be a recent JPEG under 512 KiB and 1920px";

fn strict_base64(image: &str) -> bool {
    let body = image.trim_end_matches('=');
    !body.is_empty()
        && image.len() - body.len() <= 2
        && body
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'+' || c == b'/')
}
/// Validate an upload body against trusted server time `now` (ms).
pub(crate) fn validate(value: Value, now: i64) -> Result<(Capture, i64), ApiError> {
    let image = value.get("image").and_then(Value::as_str);
    let image = image.filter(|s| s.len() <= 700_000 && strict_base64(s));
    let time = value.get("captured_at").and_then(Value::as_str);
    let (Some(image), Some(time)) = (image, time) else {
        return Err(ApiError::new(400, SHAPE));
    };
    let parsed = DateTime::parse_from_rfc3339(time).map(|t| t.timestamp_millis());
    let fresh = parsed
        .as_ref()
        .is_ok_and(|t| now - t <= 300_000 && t - now <= 60_000);
    let bytes = STANDARD.decode(image).unwrap_or_default();
    let sized = (24..=512 * 1024).contains(&bytes.len());
    if !fresh || !sized || STANDARD.encode(&bytes) != image || !jpeg::bounded(&bytes) {
        return Err(ApiError::new(400, IMAGE));
    }
    let (trigger, request_id) = frame_fields::parse(&value)?;
    let millis = parsed.unwrap_or_default();
    let captured_at = DateTime::from_timestamp_millis(millis)
        .map(|t| t.to_rfc3339_opts(SecondsFormat::Millis, true))
        .ok_or_else(|| ApiError::new(400, IMAGE))?;
    let image = image.to_string();
    Ok((
        Capture {
            image,
            captured_at,
            trigger,
            request_id,
        },
        millis,
    ))
}
