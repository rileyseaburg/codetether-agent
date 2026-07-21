//! Projection of task activity observed during A2A status polling.

pub(super) fn record(name: &str, owner: Option<&str>, task: &crate::a2a::types::Task) {
    let Some(events) = task
        .metadata
        .get("activity")
        .and_then(serde_json::Value::as_array)
    else {
        return;
    };
    super::super::observation::record_activity(name, owner, events);
}
