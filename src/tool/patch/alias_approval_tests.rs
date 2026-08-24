//! Compatibility alias preserves reviewed policy identity through patch claim.

use super::test_support::{execute, sample_patch, seed};
use crate::approval::{ApprovalStore, test_env::{ScopedEnv, lock_env}};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn patch_alias_approval_applies_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let dir = tempfile::tempdir().expect("workspace");
    seed(dir.path());
    let mut args = json!({"patch": sample_patch()});
    let blocked = crate::tool::alias::scoped("patch", execute(dir.path(), args.clone())).await;
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("approval id")
        .to_string();
    let store = ApprovalStore::open_default().expect("store");
    let request = store.request(&id).expect("lookup").expect("request");
    assert_eq!(request.tool, "patch");
    store.approve(&id, "test", "apply").expect("approve");
    args["approval_id"] = json!(id);

    let first = crate::tool::alias::scoped("patch", execute(dir.path(), args.clone())).await;
    assert!(first.success, "{first:?}");
    let replay = crate::tool::alias::scoped("patch", execute(dir.path(), args)).await;
    assert!(!replay.success);
    assert_eq!(
        replay.metadata["error_code"],
        "APPROVAL_RECEIPT_REJECTED"
    );
}