use crate::images::protocol_error::classify;
use anyhow::anyhow;
use codetether_gemini_web_protocol::images::parse_response;

#[test]
fn fixed_codes_survive_but_arbitrary_content_does_not() {
    assert_eq!(
        classify(anyhow!("truncated_rpc_stream")).to_string(),
        "image_generation_protocol_error: truncated_rpc_stream"
    );
    assert_eq!(
        classify(anyhow!("https://example.invalid/?secret=private")).to_string(),
        "image_generation_protocol_error: unknown"
    );
}

#[test]
fn malformed_frame_keeps_json_location_without_its_contents() {
    let raw = r#"[["secret-signed-url",secret-cookie]]"#;
    let bytes = format!(")]}}'\n{}\n{raw}\n", raw.len() + 2);
    let error = parse_response(bytes.as_bytes()).unwrap_err();
    let json = error.downcast_ref::<serde_json::Error>().unwrap();
    let expected = format!(
        "image_generation_protocol_error: invalid_batch_json ({:?}, line={}, column={})",
        json.classify(),
        json.line(),
        json.column()
    );
    let message = classify(error).to_string();
    assert_eq!(message, expected);
    assert!(!message.contains("secret"));
}
