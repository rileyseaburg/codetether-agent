//! Screenshot producer regression tests without a browser process.

use super::*;
use base64::{Engine, engine::general_purpose::STANDARD};

#[tokio::test]
async fn screenshot_result_preserves_bytes_and_path_without_base64_text() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested/screenshot.png");
    let input = serde_json::from_value(json!({
        "action": "screenshot", "path": path
    }))
    .unwrap();
    let bytes = STANDARD.decode("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+jRZkAAAAASUVORK5CYII=").unwrap();
    let result = crate::tool::browserctl::response::success_result(
        &input,
        crate::browser::BrowserOutput::Screenshot(ScreenshotData {
            bytes: bytes.clone(),
        }),
    )
    .await
    .unwrap();
    assert!(result.success);
    assert_eq!(tokio::fs::read(&path).await.unwrap(), bytes);
    assert_eq!(result.metadata["path"], json!(path));
    assert_eq!(result.metadata["file"]["exists"], true);
    assert_eq!(result.metadata["file"]["absolute"], true);
    let image = &result.metadata["image_data_url"];
    assert_eq!(image["mime_type"], "image/png");
    let payload = image["data_url"]
        .as_str()
        .unwrap()
        .strip_prefix("data:image/png;base64,")
        .unwrap();
    assert_eq!(STANDARD.decode(payload).unwrap(), bytes);
    assert_eq!(
        serde_json::from_str::<Value>(&result.output).unwrap(),
        json!({"path": path})
    );
    assert!(!result.output.contains("base64"));
}
