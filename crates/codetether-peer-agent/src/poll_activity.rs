//! Projection of task activity observed during A2A status polling.

pub(super) fn record(name: &str, owner: Option<&str>, task: &codetether_a2a_core::Task) {
    let Some(events) = task
        .metadata
        .get("activity")
        .and_then(serde_json::Value::as_array)
    else {
        return;
    };
    super::super::observation::record_activity(name, owner, events);
}
