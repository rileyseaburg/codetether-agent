//! Runtime policy invocation arguments for special worker tasks.

use serde_json::Value;

/// Extract runtime approval arguments using the shared metadata lookup order.
///
/// # Arguments
/// * `metadata` - Task metadata, including optional nested forage fields.
/// # Returns
/// An object containing the selected approval ID when present.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::task_config::policy_args::from_metadata;
/// let metadata = serde_json::json!({"approval_id": "one"}).as_object().unwrap().clone();
/// assert_eq!(from_metadata(&metadata)["approval_id"], "one");
/// ```
pub fn from_metadata(metadata: &serde_json::Map<String, Value>) -> Value {
    let mut args = serde_json::Map::new();
    if let Some(value) = super::metadata_lookup(metadata, "approval_id") {
        args.insert("approval_id".to_string(), value.clone());
    }
    Value::Object(args)
}

#[cfg(test)]
#[path = "policy_args_tests.rs"]
mod tests;
