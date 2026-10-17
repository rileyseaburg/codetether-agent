//! Pod request schema assertions.
pub(super) fn check(body: &serde_json::Value) {
    assert_eq!(body["metadata"]["namespace"], "fixture");
    assert_eq!(body["metadata"]["labels"]["app"], "custom");
    assert_eq!(
        body["metadata"]["labels"]["codetether.run/subagent-id"],
        "Task_A123"
    );
    assert_eq!(
        body["metadata"]["annotations"]["codetether.run/operation"],
        "spawn-subagent"
    );
    let c = &body["spec"]["containers"][0];
    assert_eq!(c["image"], "fixture:v1");
    assert_eq!(c["command"][0], "agent");
    assert_eq!(c["args"][0], "run");
    assert_eq!(c["resources"]["limits"]["cpu"], "1000m");
    assert!(
        c["env"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e["name"] == "CODETETHER_SUBAGENT_ID" && e["value"] == "Task_A/123")
    );
}
