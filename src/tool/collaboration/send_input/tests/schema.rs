//! Schema coverage for every structured input variant.
use super::super::item::InputItem;
use serde_json::json;

#[test]
fn schema_exposes_typed_required_fields() {
    let schema = super::parameters();
    let variants = schema["properties"]["items"]["items"]["anyOf"]
        .as_array()
        .unwrap();
    let cases = [
        json!({"type":"text", "text":"hello"}),
        json!({"type":"image", "image_url":"data:image/png;base64,AA=="}),
        json!({"type":"local_image", "path":"image.png"}),
        json!({"type":"audio", "audio_url":"data:audio/wav;base64,AA=="}),
        json!({"type":"local_audio", "path":"audio.wav"}),
        json!({"type":"skill", "name":"skill", "path":"SKILL.md"}),
        json!({"type":"mention", "name":"name", "path":"path"}),
    ];
    assert_eq!(variants.len(), cases.len());
    for case in cases {
        let variant = variants
            .iter()
            .find(|v| v["properties"]["type"]["enum"][0] == case["type"])
            .unwrap();
        for (key, _) in case.as_object().unwrap() {
            assert_eq!(variant["properties"][key]["type"], "string");
            assert!(
                variant["required"]
                    .as_array()
                    .unwrap()
                    .contains(&json!(key))
            );
        }
        assert_eq!(variant["additionalProperties"], false);
        if case["type"] == "image" {
            let description = variant["description"].as_str().unwrap();
            assert!(description.contains("HTTP(S)"));
            assert!(description.contains("without fetching"));
        }
        assert!(serde_json::from_value::<InputItem>(case).is_ok());
    }
}
