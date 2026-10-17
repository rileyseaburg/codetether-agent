//! Create and pair an isolated session through its HTTP routes.
use super::{OWNER, app, call};
use serde_json::json;

pub async fn paired() -> (axum::Router, String, String) {
    let app = app();
    let input = json!({"model":"p/vision", "prompt":"Describe", "interval_seconds":30});
    let (status, created) = call(
        &app,
        "POST",
        "/companion/sessions",
        Some(OWNER),
        Some(input),
    )
    .await;
    assert_eq!(status, 200);
    let (status, paired) = call(
        &app,
        "POST",
        "/companion/pair",
        None,
        Some(json!({"code":created["code"]})),
    )
    .await;
    assert_eq!(status, 200);
    (
        app,
        created["id"].as_str().unwrap().into(),
        paired["device_token"].as_str().unwrap().into(),
    )
}
