//! Runtime-input envelope fields, ignored rather than trusted by pure planning.
//!
//! Keep this exact list aligned with codetether-runtime-input context, provenance
//! and session enrichment. Do not accept arbitrary __ct_* fields or forward
//! identities, signatures, leases or grants into a suggested exec_command.
pub(super) fn is_runtime_field(key: &str) -> bool {
    matches!(
        key,
        "__ct_agent_identity_id"
            | "__ct_agent_name"
            | "__ct_attempt_id"
            | "__ct_current_model"
            | "__ct_key_id"
            | "__ct_lease_owner"
            | "__ct_origin"
            | "__ct_parent_workspace"
            | "__ct_prior_context_allowed"
            | "__ct_provenance_id"
            | "__ct_run_id"
            | "__ct_session_id"
            | "__ct_signature"
            | "__ct_task_id"
            | "__ct_tenant_id"
            | "__ct_worker_id"
    )
}
