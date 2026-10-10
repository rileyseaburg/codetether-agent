//! HTTP contract tests: owner/device split, pairing, commands, frames, stop.
mod common;
use common::{OWNER, app, call};
use serde_json::json;

#[tokio::test]
async fn owner_device_split_and_lifecycle() {
    let app = app();
    let input = json!({"model": "p/vision", "prompt": "Describe", "interval_seconds": 30});
    let sessions = "/companion/sessions";
    assert_eq!(
        call(&app, "POST", sessions, None, Some(input.clone()))
            .await
            .0,
        401
    );
    let (status, created) = call(&app, "POST", sessions, Some(OWNER), Some(input)).await;
    assert_eq!(status, 200);
    let id = created["id"].as_str().unwrap().to_string();
    let code = Some(json!({"code": created["code"]}));
    let (status, paired) = call(&app, "POST", "/companion/pair", None, code).await;
    assert_eq!(status, 200);
    let device = paired["device_token"].as_str().unwrap().to_string();
    let commands = format!("/companion/sessions/{id}/commands");
    assert_eq!(call(&app, "GET", &commands, Some(OWNER), None).await.0, 401);
    let idle = call(&app, "GET", &commands, Some(&device), None).await;
    assert_eq!(idle, (200, json!({"request_id": null})));
    let request = format!("/companion/sessions/{id}/request");
    let question = Some(json!({"question": "What?"}));
    assert_eq!(
        call(&app, "POST", &request, Some(&device), question.clone())
            .await
            .0,
        401
    );
    let (status, queued) = call(&app, "POST", &request, Some(OWNER), question).await;
    assert_eq!(status, 202);
    let (_, polled) = call(&app, "GET", &commands, Some(&device), None).await;
    assert_eq!(polled, json!({"request_id": queued["request_id"]}));
    let frames = format!("/companion/sessions/{id}/frames");
    let stale = Some(json!({"image": "AAAA", "captured_at": "2020-01-01T00:00:00Z"}));
    assert_eq!(
        call(&app, "POST", &frames, Some(&device), stale).await.0,
        400
    );
    let session = format!("/companion/sessions/{id}");
    assert_eq!(
        call(&app, "DELETE", &session, Some(&device), None).await.0,
        401
    );
    let stopped = call(&app, "DELETE", &session, Some(OWNER), None).await;
    assert_eq!(stopped, (200, json!({"stopped": true})));
    assert_eq!(
        call(&app, "GET", &commands, Some(&device), None).await.0,
        410
    );
}
