//! Fixture capture exercises fallback, private artifacts, and image attachments.

use super::*;
use base64::Engine;

#[tokio::test]
async fn capture_falls_back_and_attaches_the_retained_original() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = dir.path().join("fixture with spaces.png");
    std::fs::write(&fixture, png()).unwrap();
    let bad = script(dir.path(), "bad capture", "printf broken > \"$2\"");
    let good = script(
        dir.path(),
        "good capture",
        &format!("cp '{}' \"$2\"", fixture.display()),
    );
    let result = snapshot::capture(
        Session::X11,
        &[(Backend::Scrot, bad), (Backend::Scrot, good)],
    )
    .await
    .unwrap();
    assert!(result.success);
    let data: serde_json::Value = serde_json::from_str(&result.output).unwrap();
    let path = PathBuf::from(data["path"].as_str().unwrap());
    assert_eq!(std::fs::read(&path).unwrap(), png());
    assert_eq!(data["coordinate_space"], "captured_image_pixels");
    assert!(data["cursor"].is_null());
    assert_eq!(data["width"], 80);
    assert_eq!(data["preview"]["image_to_original_scale_x"], 1.0);
    let attachment = &result.metadata["image_data_url"];
    assert_eq!(attachment["mime_type"], "image/png");
    let encoded = attachment["data_url"]
        .as_str()
        .unwrap()
        .strip_prefix("data:image/png;base64,")
        .unwrap();
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .unwrap(),
        png()
    );
    assert_eq!(
        std::fs::metadata(path.parent().unwrap())
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
}
