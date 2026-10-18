//! GET/PUT/DELETE share the harness store; clearing restores persistent fallback.
use super::support::{call, setup};
use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn verifier_model_api_sets_reads_and_clears_runtime_selection() {
    let (app, selection, _) = setup();
    let (status, initial) = call(&app, "GET", None, true).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(initial["selected_model"], "openai-codex/environment");
    assert_eq!(initial["source"], "environment");
    assert!(initial["latest_verification"].is_null());
    let (status, set) = call(
        &app,
        "PUT",
        Some(json!({"model":"  openai-codex/requested:high  "})),
        true,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        selection.get().as_deref(),
        Some("openai-codex/requested:high")
    );
    assert_eq!(set["selected_model"], "openai-codex/requested:high");
    assert_eq!(set["source"], "runtime");
    assert_eq!(set["persisted"], false);
    assert_eq!(set["identity_source"], "harness_configuration");
    let (_, read) = call(&app, "GET", None, true).await;
    assert_eq!(read["runtime_model"], set["runtime_model"]);
    let (status, cleared) = call(&app, "DELETE", None, true).await;
    assert_eq!(status, StatusCode::OK);
    assert!(selection.get().is_none());
    assert_eq!(cleared["selected_model"], "openai-codex/environment");
    assert_eq!(cleared["source"], "environment");
}
