//! Task payload access helpers.
//!
//! A2A task payloads sometimes nest fields below `task`, so these helpers keep
//! that compatibility logic away from the main task-processing functions.
//!
//! # Examples
//!
//! ```ignore
//! let id = task_str(&payload, "id");
//! ```

/// Looks up a raw task field from either the top-level object or `task`.
///
/// This allows the worker to accept legacy and normalized payload shapes.
///
/// # Examples
///
/// ```ignore
/// let value = task_value(&payload, "id");
/// assert!(value.is_some());
/// ```
pub(super) fn task_value<'a>(
    task: &'a serde_json::Value,
    key: &str,
) -> Option<&'a serde_json::Value> {
    task.get("task")
        .and_then(|nested| nested.get(key))
        .or_else(|| task.get(key))
}

/// Reads a string field from a task payload.
///
/// The worker uses this when routing and reporting on task metadata.
///
/// # Examples
///
/// ```ignore
/// let title = task_str(&payload, "title");
/// ```
pub(super) fn task_str<'a>(task: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    task_value(task, key).and_then(serde_json::Value::as_str)
}

/// Extracts the metadata object from a task payload.
///
/// Missing metadata returns an empty map so callers can use it without extra
/// branching.
///
/// # Examples
///
/// ```ignore
/// let metadata = task_metadata(&payload);
/// assert!(metadata.is_object() || metadata.is_empty());
/// ```
pub(super) fn task_metadata(
    task: &serde_json::Value,
) -> serde_json::Map<String, serde_json::Value> {
    task_value(task, "metadata")
        .and_then(serde_json::Value::as_object)
        .cloned()
        .unwrap_or_default()
}
