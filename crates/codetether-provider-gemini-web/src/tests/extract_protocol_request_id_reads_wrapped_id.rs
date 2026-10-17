use super::*;

#[test]
fn extract_protocol_request_id_reads_wrapped_id() {
    let raw = r#"[["wrb.fr",null,"[null,[null,\"r_52bc718fbddfc769\"],null]"]]"#;
    assert_eq!(
        GeminiWebProvider::extract_protocol_request_id(raw).as_deref(),
        Some("r_52bc718fbddfc769")
    );
}
