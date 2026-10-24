use codetether_gemini_web_protocol::images::{IMAGE_MODEL_ID, model_header};
use serde_json::{Value, json};

#[test]
fn image_header_uses_observed_selector_and_fresh_uuid() {
    let value: Value = serde_json::from_str(&model_header("build\"雪")).unwrap();
    assert_eq!(value[4], IMAGE_MODEL_ID);
    assert_eq!(value[8], json!([4, 5, 6, 8, 16, 19, 20]));
    assert_eq!(value[11], 2);
    assert_eq!(value[14], 5);
    assert_eq!(value[17], json!(["build\"雪"]));
    assert!(uuid::Uuid::parse_str(value[16].as_str().unwrap()).is_ok());
    let other: Value = serde_json::from_str(&model_header("build")).unwrap();
    assert_ne!(value[16], other[16]);
}
