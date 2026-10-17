//! Extension header and request timestamp identifiers.
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};
pub(crate) fn extension_header(mode_id: &str) -> String {
    let v: Value = json!([
        1,
        null,
        null,
        null,
        mode_id,
        null,
        null,
        0,
        [4],
        null,
        null,
        3
    ]);
    serde_json::to_string(&v).unwrap_or_default()
}
pub(crate) fn request_id() -> String {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        % 900_000
        + 100_000)
        .to_string()
}
