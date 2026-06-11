use super::{RuntimeToolPolicy, ToolPolicyOutcome};
use crate::approval::test_env::ENV_LOCK;
use crate::config::Config;

struct EnvGuard;

impl EnvGuard {
    fn data_dir(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[test]
fn approval_required_result_includes_request_id() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    let policy = RuntimeToolPolicy::from_config(&Config::default());
    let decision = policy.decide_tool("apply_patch");

    let result = super::result::blocking_result_with_approval_request(
        "apply_patch",
        decision,
        "write",
        "src/lib.rs",
    )
    .expect("blocking result");
    let request_id = result.metadata["approval_request_id"]
        .as_str()
        .expect("request id");

    assert_eq!(decision.outcome, ToolPolicyOutcome::RequireApproval);
    assert!(!request_id.is_empty());
    let output: serde_json::Value = serde_json::from_str(&result.output).expect("json output");
    assert_eq!(output["error"]["approval_request_id"], request_id);
    assert_eq!(output["error"]["example"]["approval_id"], request_id);
    assert!(output["error"]["why"].as_str().is_some());
    assert!(
        output["error"]["change_prompting"]
            .as_str()
            .expect("prompting hint")
            .contains("access_mode=approve")
    );
    assert!(data.path().join("approvals/approvals.jsonl").exists());
}
