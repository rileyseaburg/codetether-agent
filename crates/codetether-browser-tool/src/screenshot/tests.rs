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
    let bytes = ScreenshotData::from_ppm(b"P6\n1 1\n255\n\xff\x00\x00")
        .unwrap()
        .bytes;
    assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let result = crate::response::success_result(
        &input,
        codetether_browser::BrowserOutput::Screenshot(ScreenshotData {
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
