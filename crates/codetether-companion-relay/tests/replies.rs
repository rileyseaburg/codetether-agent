//! Current route visibility: reply/typed handlers are not whitelisted by paths.
mod common;
use common::{OWNER, call, session::paired};
use serde_json::json;

#[tokio::test]
async fn reply_and_typed_paths_remain_unmatched() {
    let (app, id, token) = paired().await;
    for (action, auth, value) in [
        ("reply", OWNER, json!({"text":"hello"})),
        ("typed", token.as_str(), json!({"reply_id":"unknown"})),
    ] {
        let path = format!("/companion/sessions/{id}/{action}");
        let response = call(&app, "POST", &path, Some(auth), Some(value)).await;
        assert_eq!(response, (404, json!({"error":"Route not found"})));
    }
    let commands = format!("/companion/sessions/{id}/commands");
    assert!(call(&app, "GET", &commands, Some(&token), None).await.1["reply"].is_null());
}
