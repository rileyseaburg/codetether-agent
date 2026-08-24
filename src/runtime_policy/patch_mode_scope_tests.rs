use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[test]
fn preview_approval_cannot_authorize_patch_write() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let patch = "*** Begin Patch\n*** Add File: marker\n+written\n*** End Patch";
    let preview = json!({
        "patch": patch,
        "dry_run": true,
        "__ct_parent_workspace": data.path(),
    });
    let preview_scope = super::invocation_scope::for_tool("apply_patch", &preview);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("apply_patch", "write", &preview_scope.resource, "preview")
        .expect("request");
    store
        .approve(&request.id, "test", "preview only")
        .expect("approve");
    let write = json!({
        "patch": patch,
        "dry_run": false,
        "approval_id": request.id,
        "__ct_parent_workspace": data.path(),
    });

    assert_ne!(
        preview_scope.resource,
        super::invocation_scope::for_tool("apply_patch", &write).resource
    );
    assert!(!super::approved_invocation("apply_patch", &write));
}
