use super::{ApprovalStore, Config, evaluate_tool_invocation_with_config, session_grants};
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use serde_json::{Value, json};

#[test]
fn generic_session_retry_still_consumes_one_time_receipt() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({
        "path": "file.txt",
        "content": "test",
        "__ct_session_id": "claim-session"
    });
    let blocked = evaluate_tool_invocation_with_config(&Config::default(), "generic_mutation", &args)
        .expect("approval required");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    let receipt = ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "session")
        .expect("approve");
    session_grants::grant(&receipt);
    args["approval_id"] = Value::String(id.to_string());

    assert!(evaluate_tool_invocation_with_config(&Config::default(), "generic_mutation", &args).is_none());
    let replay = evaluate_tool_invocation_with_config(&Config::default(), "generic_mutation", &args)
        .expect("spent receipt blocked");
    assert_eq!(replay.metadata["error_code"], "APPROVAL_RECEIPT_REJECTED");
    args.as_object_mut().unwrap().remove("approval_id");
    assert!(evaluate_tool_invocation_with_config(&Config::default(), "generic_mutation", &args).is_none());
}