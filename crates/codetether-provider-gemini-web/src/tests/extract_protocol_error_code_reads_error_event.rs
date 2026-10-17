use super::*;

#[test]
fn extract_protocol_error_code_reads_error_event() {
    let raw = r#"
)]}'
25
[["e",5,null,null,469]]
"#;
    assert_eq!(
        GeminiWebProvider::extract_protocol_error_code(raw),
        Some(469)
    );
}
