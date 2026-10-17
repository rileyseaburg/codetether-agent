//! Post-clone task metadata helpers.

pub(super) fn worker_should_enqueue_post_clone_task(
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> bool {
    metadata
        .get("post_clone_task")
        .and_then(|value| value.as_object())
        .is_some()
}
