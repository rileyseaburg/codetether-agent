//! Approval consumption remains durable across store instances.

use crate::approval::ApprovalStore;

#[test]
fn claimed_receipt_remains_consumed_after_store_reopen() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let request = store
        .create_request("exec_command", "execute", "exact-command", "once")
        .expect("request");
    store
        .approve(&request.id, "test", "allow")
        .expect("approve");
    store
        .claim(
            &request.id,
            "exec_command",
            "execute",
            "exact-command",
            "test",
        )
        .expect("claim");
    drop(store);
    let reopened = ApprovalStore::open(dir.path()).expect("reopen");
    assert!(
        reopened
            .verify(&request.id, "exec_command", "execute", "exact-command")
            .is_err()
    );
}
