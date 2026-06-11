use crate::approval::{ApprovalStore, session_command_grants, test_env::ENV_LOCK};
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

#[path = "tests_env.rs"]
mod env;

#[tokio::test]
async fn server_session_decision_grants_prefix() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let data = tempfile::tempdir().expect("tempdir");
    let _env = env::EnvGuard::data_dir(data.path());
    session_command_grants::reset();
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:abc", "needs approval")
        .expect("request");
    session_command_grants::remember_request(&request.id, vec!["cargo test".into()]);
    let decision = Request::builder()
        .method("POST")
        .uri(format!("/v1/tools/approvals/{}/decision", request.id))
        .header("content-type", "application/json")
        .body(axum::body::Body::from(
            json!({"decision": "approved_for_session"}).to_string(),
        ))
        .expect("decision");

    let response = super::router::<()>()
        .oneshot(decision)
        .await
        .expect("route");

    assert_eq!(response.status(), StatusCode::OK);
    assert!(session_command_grants::allowed("cargo test --lib server"));
}
