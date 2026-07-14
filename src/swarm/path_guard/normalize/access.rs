//! Utilities for accessing nested values inside `serde_json::Value` trees.
//!
//! This module provides path-based lookup helpers for JSON-like data. Paths use
//! dot-separated segments, where each segment selects either an object key or an
//! array index depending on the current value being traversed.

use serde_json::Value;

/// Returns a mutable reference to a nested JSON value selected by a dot-separated path.
///
/// Each path segment is interpreted according to the current JSON value:
///
/// - For `Value::Object`, the segment is used as an object key.
/// - For `Value::Array`, the segment must parse as a `usize` and is used as an array index.
/// - For all other value types, traversal fails.
///
/// Returns `None` if any object key is missing, an array index is invalid or out of bounds,
/// an array segment cannot be parsed as `usize`, or traversal reaches a non-container value
/// before the path is exhausted.
///
/// The returned reference allows callers to modify the selected nested value in place.
///
/// # Parameters
///
/// - `value`: The root JSON value to traverse and potentially mutate.
/// - `spec`: A dot-separated path such as `"users.0.name"`.
///
/// # Returns
///
/// A mutable reference to the selected nested `Value`, or `None` if the path cannot be
/// resolved.
///
/// # Examples
///
/// /// use serde_json::json;
///
/// let mut data = json!({
///     "users": [
///         { "name": "Ada" }
///     ]
/// });
///
/// if let Some(name) = get_mut_path(&mut data, "users.0.name") {
///     *name = json!("Grace");
/// }
///
/// assert_eq!(data["users"][0]["name"], "Grace");
pub fn get_mut_path<'a>(value: &'a mut Value, spec: &str) -> Option<&'a mut Value> {
    let mut current = value;
    for part in spec.split('.') {
        current = match current {
            Value::Object(map) => map.get_mut(part)?,
            Value::Array(items) => items.get_mut(part.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(current)
}