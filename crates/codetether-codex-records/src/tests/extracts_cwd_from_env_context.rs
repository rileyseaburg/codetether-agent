use crate::*;
use serde_json::Value;

#[test]
fn extracts_cwd_from_env_context() {
    let raw = r#"{"type":"message","role":"user","content":[{"type":"input_text","text":"<environment_context>\n  <cwd>/home/riley/spotlessbinco</cwd>\n</environment_context>"}]}"#;
    let value: Value = serde_json::from_str(raw).unwrap();
    assert_eq!(
        extract_cwd_from_env_context(&value).as_deref(),
        Some("/home/riley/spotlessbinco")
    );
}
