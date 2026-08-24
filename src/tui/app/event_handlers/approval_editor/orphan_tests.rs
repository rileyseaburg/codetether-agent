use crate::approval::{ApprovalEvent, ApprovalStore, test_env::lock_env};

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[test]
fn missing_waiter_revokes_revised_patch_approval() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    let _guard = Guard;
    let store = ApprovalStore::open_default().expect("store");
    let patch = "--- a/file.txt\n+++ b/file.txt\n@@ -1 +1 @@\n-old\n+new\n";
    let resource = crate::tool::patch::approval_resource_for_root(data.path(), patch);
    let original = store
        .create_request("apply_patch", "write", &resource, "test")
        .expect("request");

    assert!(super::finish::apply(&original.id, patch).is_err());

    let revised = store
        .events()
        .unwrap()
        .into_iter()
        .filter_map(|event| match event {
            ApprovalEvent::Request { request } if request.id != original.id => Some(request),
            ApprovalEvent::Request { .. } | ApprovalEvent::Decision { .. } => None,
        })
        .next()
        .expect("revised request");
    let resource = crate::tool::patch::approval_resource_for_root(data.path(), patch);
    let result = store.verify(&revised.id, "apply_patch", "write", &resource);
    assert!(result.is_err());
}
