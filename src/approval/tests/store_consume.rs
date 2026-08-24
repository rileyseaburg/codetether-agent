use crate::approval::ApprovalStore;

#[test]
fn consumed_approval_cannot_be_replayed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let request = store
        .create_request("exec_command", "execute", "scope", "test")
        .expect("request");
    let receipt = store
        .approve(&request.id, "test", "approved once")
        .expect("approve");
    assert!(
        store
            .claim(&request.id, "read", "execute", "scope", "unrelated")
            .is_err()
    );
    store
        .claim(
            &request.id,
            "exec_command",
            "execute",
            "scope",
            "test-runtime",
        )
        .expect("first claim");
    let verified = store
        .verify_receipt(&receipt, "exec_command", "execute", "scope")
        .expect("historical receipt");
    assert_eq!(verified, receipt);

    let replay = store.claim(
        &request.id,
        "exec_command",
        "execute",
        "scope",
        "test-runtime",
    );
    assert!(replay.is_err());
    assert!(replay.unwrap_err().to_string().contains("denied"));
}
