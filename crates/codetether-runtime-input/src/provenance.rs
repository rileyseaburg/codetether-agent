//! Provenance fields and optional signatures preserve supplied field precedence.
use super::insert_field;
use codetether_provenance::{ExecutionProvenance, sign_provenance};
use serde_json::{Map, Value, json};

/// Insert the standard provenance fields into a tool input object.
///
/// # Examples
///
/// ```rust
/// use codetether_provenance::{ExecutionOrigin, ExecutionProvenance};
/// use codetether_runtime_input::insert_provenance_fields;
/// use serde_json::{Map, Value};
///
/// let mut obj = Map::<String, Value>::new();
/// let provenance = ExecutionProvenance::for_operation("agent-build", ExecutionOrigin::Swarm);
/// insert_provenance_fields(&mut obj, &provenance);
///
/// assert_eq!(obj["__ct_origin"], "swarm");
/// assert!(obj.get("__ct_provenance_id").is_some());
/// ```
/// # Arguments
/// Receives a tool object and execution provenance.
/// # Returns
/// Adds missing provenance fields and an optional signature.
pub fn insert_provenance_fields(obj: &mut Map<String, Value>, provenance: &ExecutionProvenance) {
    for (key, value) in [
        (
            "__ct_provenance_id",
            Some(provenance.provenance_id.as_str()),
        ),
        ("__ct_origin", Some(provenance.identity.origin.as_str())),
        ("__ct_worker_id", provenance.identity.worker_id.as_deref()),
        ("__ct_task_id", provenance.task_id.as_deref()),
        ("__ct_run_id", provenance.run_id.as_deref()),
        ("__ct_attempt_id", provenance.attempt_id.as_deref()),
        ("__ct_tenant_id", provenance.identity.tenant_id.as_deref()),
        (
            "__ct_agent_identity_id",
            provenance.identity.agent_identity_id.as_deref(),
        ),
        (
            "__ct_agent_name",
            Some(provenance.identity.agent_name.as_str()),
        ),
        ("__ct_key_id", provenance.identity.key_id.as_deref()),
    ] {
        insert_provenance_field(obj, key, value);
    }
    if let Some(signature) = sign_provenance(provenance) {
        obj.entry("__ct_signature".to_string())
            .or_insert_with(|| json!(signature));
    }
}

/// Insert a single provenance field into a tool input object.
///
/// # Examples
///
/// ```rust
/// use codetether_runtime_input::insert_provenance_field;
/// use serde_json::{Map, Value};
///
/// let mut obj = Map::<String, Value>::new();
/// insert_provenance_field(&mut obj, "__ct_origin", Some("swarm"));
/// insert_provenance_field(&mut obj, "__ct_missing", None);
///
/// assert_eq!(obj["__ct_origin"], "swarm");
/// assert!(!obj.contains_key("__ct_missing"));
/// ```
/// # Arguments
/// Receives a tool object, field key and optional value.
/// # Returns
/// Adds the value only when present and its key is absent.
pub fn insert_provenance_field(obj: &mut Map<String, Value>, key: &str, value: Option<&str>) {
    insert_field(obj, key, value);
}
