use super::render;
use crate::provider::ContentPart;

#[test]
fn renders_file_with_exact_path_and_mime_type() {
    let rendered = render(&ContentPart::File {
        path: "/workspace/failure.json".to_string(),
        mime_type: Some("application/json".to_string()),
    })
    .unwrap();

    assert!(rendered.contains("use this exact path"));
    assert!(rendered.contains("/workspace/failure.json"));
    assert!(rendered.contains("application/json"));
}

#[test]
fn escapes_file_path_as_json() {
    let rendered = render(&ContentPart::File {
        path: "quoted\"path.json".to_string(),
        mime_type: None,
    })
    .unwrap();

    assert!(rendered.contains(r#"quoted\"path.json"#));
    assert!(!rendered.contains("<tool_call>"));
}
