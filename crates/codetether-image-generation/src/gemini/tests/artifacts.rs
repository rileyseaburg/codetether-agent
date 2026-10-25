//! Native Gemini artifacts preserve MIME and cannot overwrite prior versions.
use super::super::artifacts;
use codetether_gemini_web_transport::images::{GeneratedImage, ImageGeneration};

#[test]
fn saves_jpeg_without_claiming_png() {
    let root = tempfile::tempdir().unwrap();
    let generation = ImageGeneration {
        images: vec![GeneratedImage {
            bytes: vec![1, 2, 3],
            mime_type: "image/jpeg".into(),
            extension: "jpg".into(),
            width: 1,
            height: 1,
        }],
        continuation: None,
    };
    let saved = artifacts::save(&generation, root.path().join("version")).unwrap();
    assert_eq!(std::fs::read(&saved.images[0]).unwrap(), [1, 2, 3]);
    assert_eq!(saved.images[0].extension().unwrap(), "jpg");
    assert!(artifacts::save(&generation, root.path().join("version")).is_err());
    let result = crate::result::generated_typed(
        "data:image/jpeg;base64,AQID".into(),
        Some(&saved.images[0]),
        "image/jpeg",
    );
    assert_eq!(result.metadata["image_data_url"]["mime_type"], "image/jpeg");
}

#[test]
fn rejects_remote_filename_material() {
    let root = tempfile::tempdir().unwrap();
    let generation = ImageGeneration {
        images: vec![GeneratedImage {
            bytes: vec![1],
            mime_type: "image/jpeg".into(),
            extension: "../unsafe".into(),
            width: 1,
            height: 1,
        }],
        continuation: None,
    };
    assert!(artifacts::save(&generation, root.path().join("version")).is_err());
}
