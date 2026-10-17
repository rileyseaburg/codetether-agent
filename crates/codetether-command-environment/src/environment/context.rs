//! Runtime context variables inherited from the session tool call.

use serde_json::Value;

const FIELDS: &[(&str, &str)] = &[
    ("__ct_current_model", "CODETETHER_CURRENT_MODEL"),
    ("__ct_provenance_id", "CODETETHER_PROVENANCE_ID"),
    ("__ct_origin", "CODETETHER_ORIGIN"),
    ("__ct_agent_name", "CODETETHER_AGENT_NAME"),
    ("__ct_agent_identity_id", "CODETETHER_AGENT_IDENTITY_ID"),
    ("__ct_key_id", "CODETETHER_KEY_ID"),
    ("__ct_signature", "CODETETHER_SIGNATURE"),
    ("__ct_tenant_id", "CODETETHER_TENANT_ID"),
    ("__ct_worker_id", "CODETETHER_WORKER_ID"),
    ("__ct_session_id", "CODETETHER_SESSION_ID"),
    ("__ct_task_id", "CODETETHER_TASK_ID"),
    ("__ct_run_id", "CODETETHER_RUN_ID"),
    ("__ct_attempt_id", "CODETETHER_ATTEMPT_ID"),
];

pub(super) fn extend(environment: &mut Vec<(String, String)>, args: &Value) {
    environment.extend(FIELDS.iter().filter_map(|(argument, variable)| {
        args.get(argument)
            .and_then(Value::as_str)
            .map(|value| ((*variable).to_string(), value.to_string()))
    }));
}
