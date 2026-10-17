//! Screenshot failure metadata regressions.

use super::*;

#[tokio::test]
async fn failed_screenshot_write_does_not_attach_image_metadata() {
    let dir = tempfile::tempdir().unwrap();
    let input = serde_json::from_value(json!({
        "action": "screenshot", "path": dir.path()
    }))
    .unwrap();
    let mut metadata = HashMap::new();
    assert!(
        write(
            &input,
            ScreenshotData {
                bytes: vec![1, 2, 3]
            },
            &mut metadata
        )
        .await
        .is_err()
    );
    assert!(metadata.is_empty());
}

#[tokio::test]
async fn screenshot_still_requires_an_output_path() {
    let input = serde_json::from_value(json!({"action": "screenshot"})).unwrap();
    let mut metadata = HashMap::new();
    assert!(
        write(&input, ScreenshotData { bytes: vec![] }, &mut metadata)
            .await
            .is_err()
    );
    assert!(metadata.is_empty());
}
