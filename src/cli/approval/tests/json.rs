use super::super::{json, records};
use crate::approval::ApprovalStore;
use serde_json::Value;

/// Parse the `list --json` output for a freshly created store.
pub(super) fn list_json(store: &ApprovalStore) -> Value {
    let records = records::load(store).expect("load");
    serde_json::from_str(&json::list(&records).expect("serialize")).expect("valid json")
}

/// `--json` exists so a caller never has to parse the padded text table. The
/// empty case must still be valid JSON, not the human-readable message.
#[test]
fn an_empty_store_serializes_as_an_empty_array() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    assert_eq!(list_json(&store), serde_json::json!([]));
}

#[test]
fn a_pending_request_serializes_every_field() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let request = store
        .create_request("bash", "execute", "echo ok", "needs approval")
        .expect("request");

    let parsed = list_json(&store);
    let entry = &parsed[0];
    assert_eq!(entry["id"], request.id.as_str());
    assert_eq!(entry["status"], "pending");
    assert_eq!(entry["tool"], "bash");
    assert_eq!(entry["action"], "execute");
    assert_eq!(entry["resource"], "echo ok");
    assert_eq!(entry["reason"], "needs approval");
    // A pending request has no decision, and null is how a caller detects that.
    assert!(entry["decision"].is_null(), "got: {entry}");
    assert!(
        entry["requested_at"]
            .as_str()
            .is_some_and(|stamp| stamp.contains('T')),
        "requested_at should be RFC 3339: {entry}"
    );
}
