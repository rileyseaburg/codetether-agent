//! Propagate provenance and Git identity into a direct command.
use codetether_command_environment::git_identity_env_from_tool_args;
use serde_json::Value;
use tokio::process::Command;
pub(super) fn configure(cmd: &mut Command, args: &Value) {
    for (key, value) in git_identity_env_from_tool_args(&args) {
        cmd.env(key, value);
    }
    for (arg_key, env_key) in [
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
    ] {
        if let Some(value) = args[arg_key].as_str() {
            cmd.env(env_key, value);
        }
    }
}
