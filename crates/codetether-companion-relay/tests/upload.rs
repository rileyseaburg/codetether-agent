//! Pausing while a frame uploads must invalidate its captured sequence.
mod common;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use common::{call, session::paired};
use tower::ServiceExt;

#[tokio::test]
async fn pause_during_body_read_rejects_the_frame() {
    let (app, id, token) = paired().await;
    let (started, reading) = tokio::sync::oneshot::channel();
    let (release, paused) = tokio::sync::oneshot::channel();
    let body = Body::from_stream(futures_util::stream::once(async move {
        started.send(()).unwrap();
        paused.await.unwrap();
        Ok::<_, std::io::Error>(frame())
    }));
    let request = Request::post(format!("/companion/sessions/{id}/frames"))
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(body)
        .unwrap();
    let uploading = tokio::spawn(app.clone().oneshot(request));
    reading.await.unwrap();
    let path = format!("/companion/sessions/{id}/pause");
    assert_eq!(call(&app, "POST", &path, Some(&token), None).await.0, 200);
    release.send(()).unwrap();
    let response = uploading.await.unwrap().unwrap();
    assert_eq!(response.status(), 409);
    let body = to_bytes(response.into_body(), 4096).await.unwrap();
    let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error["error"], "Session changed while uploading");
}

fn frame() -> String {
    use base64::{Engine, engine::general_purpose::STANDARD};
    let jpeg = [
        0xff, 0xd8, 0xff, 0xc0, 0, 11, 8, 0, 1, 0, 1, 1, 1, 0x11, 0, 0, 0, 0, 0, 0, 0, 0, 0xff,
        0xd9,
    ];
    serde_json::json!({"image":STANDARD.encode(jpeg), "captured_at":chrono::Utc::now().to_rfc3339()}).to_string()
}
