use crate::approval::{ApprovalStatus, ApprovalStore};

fn temp_store() -> (tempfile::TempDir, ApprovalStore) {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    (dir, store)
}

#[test]
fn approved_request_verifies_matching_action() {
    let (_dir, store) = temp_store();
    let request = store
        .create_request("apply_patch", "write", "src/lib.rs", "edit file")
        .expect("request");

    let receipt = store
        .approve(&request.id, "riley", "looks good")
        .expect("approve");
    let verified = store
        .verify(&request.id, "apply_patch", "write", "src/lib.rs")
        .expect("verify");
    let receipt_check = store
        .verify_receipt(&receipt, "apply_patch", "write", "src/lib.rs")
        .expect("verify receipt");

    assert_eq!(receipt, verified);
    assert_eq!(receipt, receipt_check);
    assert_eq!(verified.approval_id, request.id);
}

#[test]
fn denied_request_does_not_verify() {
    let (_dir, store) = temp_store();
    let request = store
        .create_request("bash", "execute", "cargo test", "needs shell")
        .expect("request");

    let decision = store.deny(&request.id, "riley", "too broad").expect("deny");
    let error = store
        .verify(&request.id, "bash", "execute", "cargo test")
        .expect_err("denied requests do not verify");

    assert_eq!(decision.status, ApprovalStatus::Denied);
    assert!(error.to_string().contains("denied"));
}
