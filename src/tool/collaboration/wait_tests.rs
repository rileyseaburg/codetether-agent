use super::Args;
use crate::tool::Tool;
use serde_json::json;

#[test]
fn accepts_canonical_target_lists() {
    let args: Args = serde_json::from_value(json!({"targets":["id-a","id-b"]})).unwrap();
    assert_eq!(args.requested_targets(), ["id-a", "id-b"]);
}

#[test]
fn retains_legacy_single_target_compatibility() {
    let args: Args = serde_json::from_value(json!({"target":"id-a"})).unwrap();
    assert_eq!(args.requested_targets(), ["id-a"]);
}

#[test]
fn empty_target_set_selects_v2_mailbox_wait() {
    let args: Args = serde_json::from_value(json!({})).unwrap();
    assert!(args.requested_targets().is_empty());
    assert_eq!(args.timeout_ms, 30_000);
}

#[tokio::test]
async fn v2_wait_consumes_pending_mailbox_activity() {
    let owner = "wait-v2-pending-mailbox";
    crate::tool::agent::collaboration_runtime::parent_activity::mailbox(owner);
    let args: Args = serde_json::from_value(json!({
        "timeout_ms":10000, "__ct_session_id":owner
    }))
    .unwrap();
    let result = super::run::execute(args).await.unwrap();
    let output: serde_json::Value = serde_json::from_str(&result.output).unwrap();
    assert_eq!(output["message"], "Wait completed.");
    assert_eq!(output["timed_out"], false);
}

#[test]
fn v2_schema_does_not_advertise_target_lists() {
    let properties = super::WaitAgentTool.parameters()["properties"]
        .as_object()
        .unwrap()
        .clone();
    assert_eq!(properties.keys().collect::<Vec<_>>(), ["timeout_ms"]);
}
