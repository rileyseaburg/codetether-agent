//! Advertised OCR and shadow input fields match their typed request contract.

use crate::tool::computer_use::input::{ComputerUseAction, ComputerUseInput, InputMode};
use serde_json::json;

#[test]
fn native_ocr_and_shadow_are_discoverable_without_python_packages() {
    let schema = super::parameters_schema();
    let actions = schema["properties"]["action"]["enum"].as_array().unwrap();
    for action in ["ocr", "ocr_status"] {
        assert!(actions.contains(&json!(action)));
    }
    let input: ComputerUseInput = serde_json::from_value(json!({
        "action":"ocr_status", "input_mode":"shadow"
    }))
    .unwrap();
    assert!(matches!(input.action, ComputerUseAction::OcrStatus));
    assert_eq!(input.input_mode, InputMode::Shadow);
    assert_eq!(schema["properties"]["input_mode"]["default"], "physical");
    assert_eq!(schema["properties"]["path"]["type"], "string");
    assert_eq!(schema["properties"]["language"]["type"], "string");
}

#[test]
fn ocr_empty_source_language_and_zero_hwnd_are_rejected() {
    for value in [json!({"path":""}), json!({"language":" "})] {
        let input: crate::tool::computer_use::input::OcrInput =
            serde_json::from_value(value).unwrap();
        assert!(input.validate(None).is_err());
    }
    assert!(
        crate::tool::computer_use::input::OcrInput::default()
            .validate(Some(0))
            .is_err()
    );
}
