use super::super::{ApprovalArgs, execute_with_store};
use crate::approval::{ApprovalStatus, ApprovalStore};
use clap::Parser;

fn temp_store() -> (tempfile::TempDir, ApprovalStore) {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    (dir, store)
}

#[test]
fn approve_command_records_decision() {
    let (_dir, store) = temp_store();
    let request = store
        .create_request("bash", "execute", "cargo test", "needs approval")
        .expect("request");
    run(&store, ["approval", "approve", &request.id]);
    let decision = store
        .decision(&request.id)
        .expect("decision")
        .expect("stored");
    assert_eq!(decision.status, ApprovalStatus::Approved);
}

#[test]
fn deny_command_records_decision() {
    let (_dir, store) = temp_store();
    let request = store
        .create_request("apply_patch", "write", "src/lib.rs", "needs approval")
        .expect("request");
    run(&store, ["approval", "deny", &request.id]);
    let decision = store
        .decision(&request.id)
        .expect("decision")
        .expect("stored");
    assert_eq!(decision.status, ApprovalStatus::Denied);
}

fn run<const N: usize>(store: &ApprovalStore, argv: [&str; N]) {
    let args = ApprovalArgs::try_parse_from(argv).expect("parse");
    execute_with_store(args, store).expect("execute");
}
