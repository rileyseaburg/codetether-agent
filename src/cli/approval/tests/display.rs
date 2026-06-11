use super::super::{ApprovalArgs, execute_with_store};
use crate::approval::ApprovalStore;
use clap::Parser;

#[test]
fn list_and_show_existing_request_succeed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let request = store
        .create_request("bash", "execute", "echo ok", "needs approval")
        .expect("request");

    run(&store, ["approval", "list"]);
    run(&store, ["approval", "show", &request.id]);
}

fn run<const N: usize>(store: &ApprovalStore, argv: [&str; N]) {
    let args = ApprovalArgs::try_parse_from(argv).expect("parse");
    execute_with_store(args, store).expect("execute");
}
