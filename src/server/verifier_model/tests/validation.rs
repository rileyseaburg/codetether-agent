//! Invalid or unauthenticated mutations cannot change the selected model.
use super::support::{call, setup};
use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn verifier_model_api_requires_auth_for_all_methods() {
    let (app, selection, _) = setup();
    selection.set(Some("provider/original"));
    for method in ["GET", "PUT", "DELETE"] {
        let (status, _) = call(
            &app,
            method,
            Some(json!({"model":"provider/changed"})),
            false,
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(selection.get().as_deref(), Some("provider/original"));
    }
}

#[tokio::test]
async fn verifier_model_api_rejects_bad_input_without_mutating() {
    let (app, selection, _) = setup();
    selection.set(Some("provider/original"));
    for model in [
        "",
        " ",
        "missing-provider",
        "/model",
        "provider/",
        "p/has space",
        "p/line\nbreak",
    ] {
        let (status, _) = call(&app, "PUT", Some(json!({"model":model})), true).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{model:?}");
        assert_eq!(selection.get().as_deref(), Some("provider/original"));
    }
    for body in [
        json!({}),
        json!({"model":null}),
        json!({"model":"p/m","persisted":true}),
    ] {
        let (status, _) = call(&app, "PUT", Some(body), true).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(selection.get().as_deref(), Some("provider/original"));
    }
}
