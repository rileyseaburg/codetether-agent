//! Add expiry and rotation timestamps to an imported cookie record.
use serde_json::{Value, json};
use std::collections::HashMap;
pub(super) fn insert(
    extra: &mut HashMap<String, Value>,
    earliest_expiry: Option<i64>,
    latest_expiry: Option<i64>,
) {
    if let Some(epoch) = earliest_expiry {
        extra.insert("earliest_expiry_epoch".to_string(), json!(epoch));
        if let Some(ts) = chrono::DateTime::from_timestamp(epoch, 0) {
            extra.insert(
                "earliest_expiry_rfc3339".to_string(),
                json!(ts.to_rfc3339()),
            );
        }
        extra.insert(
            "rotate_before_epoch".to_string(),
            json!(epoch.saturating_sub(24 * 60 * 60)),
        );
    }
    if let Some(epoch) = latest_expiry {
        extra.insert("latest_expiry_epoch".to_string(), json!(epoch));
    }
}
