//! Backend selection preserves existing calls and refuses ambiguous edits.
use crate::{args::ImagegenArgs, backend::ImageBackend};
use serde_json::json;

#[test]
fn defaults_to_existing_backend() {
    let args: ImagegenArgs = serde_json::from_value(json!({"prompt":"draw a lake"})).unwrap();
    assert_eq!(args.provider, ImageBackend::Openai);
    assert!(args.validate().is_ok());
}

#[test]
fn gemini_accepts_explicit_conversation_refinement() {
    let args: ImagegenArgs = serde_json::from_value(json!({
        "provider":"gemini-web", "prompt":"make it blue", "refine_from":"prior/conversation.json"
    }))
    .unwrap();
    assert_eq!(args.provider, ImageBackend::GeminiWeb);
    assert!(args.validate().is_ok());
}

#[test]
fn unsupported_or_mixed_selectors_fail_closed() {
    for input in [
        json!({"provider":"gemini-web", "prompt":"edit", "referenced_image_paths":["x.png"]}),
        json!({"provider":"gemini-web", "prompt":"edit", "num_last_images_to_include":1}),
        json!({"prompt":"edit", "refine_from":"conversation.json"}),
    ] {
        let args: ImagegenArgs = serde_json::from_value(input).unwrap();
        assert!(args.validate().is_err());
    }
}
