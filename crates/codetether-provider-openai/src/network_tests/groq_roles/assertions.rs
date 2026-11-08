//! Guidance is privileged; tool roles, IDs, arguments and content stay intact.
use super::*;

pub(super) fn messages(body: &Value, guidance_role: &str) {
    let messages = body["messages"].as_array().unwrap();
    let roles: Vec<_> = messages
        .iter()
        .map(|m| m["role"].as_str().unwrap())
        .collect();
    assert_eq!(
        roles,
        [
            "system",
            guidance_role,
            "user",
            "assistant",
            "tool",
            guidance_role
        ]
    );
    assert_eq!(messages[0]["content"], "Original instructions");
    assert_eq!(messages[1]["content"], "Initial guidance");
    assert_eq!(messages[3]["tool_calls"][0]["id"], "call-probe");
    assert_eq!(messages[3]["tool_calls"][0]["function"]["arguments"], "{}");
    assert_eq!(messages[4]["tool_call_id"], "call-probe");
    assert_eq!(messages[4]["content"], "OK");
    assert_eq!(messages[5]["content"], "Continue briefly");
}
