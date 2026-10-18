//! Unauthenticated settings requests must leave the model unchanged.
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
