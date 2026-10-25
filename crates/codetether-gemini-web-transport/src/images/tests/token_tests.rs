use crate::images::tokens;

#[test]
fn authenticated_token_wins_and_json_escapes_are_decoded() {
    let html = r#"{"thykhd":"anonymous","SNlM0e":"signed\u003ain","cfb2h":"build","FdrFJe":"sid"}"#;
    let token = tokens::parse(html, "/u/12/app").unwrap();
    assert_eq!(token.at, "signed:in");
    assert_eq!(token.prefix, "/u/12");
    assert_eq!(token.build, "build");
}

#[test]
fn anonymous_and_missing_session_metadata_are_rejected_safely() {
    let error = tokens::parse(r#"{"thykhd":"private-token"}"#, "/")
        .err()
        .unwrap();
    assert_eq!(error.to_string(), "image_session_not_authenticated");
    assert!(tokens::parse(r#"{"SNlM0e":"private-token"}"#, "/").is_err());
}
