use crate::approval::{ApprovalEvent, ApprovalStore, test_env::lock_env};
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

#[path = "tests_env.rs"]
mod env;

#[tokio::test]
async fn approval_request_and_decision_routes_round_trip() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = env::EnvGuard::data_dir(data.path());
    let app = super::router::<()>();
    let request = Request::builder()
        .method("POST")
        .uri("/v1/tools/approvals/request")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(
            json!({
                "tool": "bash", "action": "execute", "resource": "bash:abc"
            })
            .to_string(),
        ))
        .expect("request");
    let response = app.clone().oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let events = ApprovalStore::open(data.path().join("approvals"))
        .expect("store")
        .events()
        .expect("events");
    let id = match &events[0] {
        ApprovalEvent::Request { request } => request.id.as_str(),
        ApprovalEvent::Decision { .. } => panic!("request event"),
    };
    let decision = Request::builder()
        .method("POST")
        .uri(format!("/v1/tools/approvals/{id}/decision"))
        .header("content-type", "application/json")
        .body(axum::body::Body::from(
            json!({"decision": "deny"}).to_string(),
        ))
        .expect("decision");
    assert_eq!(
        app.oneshot(decision).await.expect("decision").status(),
        StatusCode::OK
    );
}
