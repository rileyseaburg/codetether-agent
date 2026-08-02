use super::super::{json, records};
use super::json::list_json;
use crate::approval::ApprovalStore;

/// The whole reason `--json` was added: a resource containing a space survives,
/// where splitting the text table's columns would truncate it at the space.
#[test]
fn a_resource_containing_spaces_round_trips() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let resource = "/tmp/dir with spaces/file.txt";
    store
        .create_request("apply_patch", "write", resource, "why")
        .expect("request");

    assert_eq!(list_json(&store)[0]["resource"], resource);
}

#[test]
fn a_decided_request_carries_its_decision() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let request = store
        .create_request("bash", "execute", "rm -rf /tmp/x", "cleanup")
        .expect("request");
    store
        .approve(&request.id, "ops", "reviewed")
        .expect("approve");

    let records = records::load(&store).expect("load");
    let record = records.first().expect("one record");
    let parsed: serde_json::Value =
        serde_json::from_str(&json::show(record).expect("serialize")).expect("valid json");

    assert_eq!(parsed["status"], "approved");
    assert_eq!(parsed["decision"]["decided_by"], "ops");
    assert_eq!(parsed["decision"]["reason"], "reviewed");
}
