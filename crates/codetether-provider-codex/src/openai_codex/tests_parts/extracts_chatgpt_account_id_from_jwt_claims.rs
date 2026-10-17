#[test]
fn extracts_chatgpt_account_id_from_jwt_claims() {
    let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"none","typ":"JWT"}"#);
    let payload = URL_SAFE_NO_PAD
        .encode(r#"{"https://api.openai.com/auth":{"chatgpt_account_id":"org_test123"}}"#);
    let jwt = format!("{header}.{payload}.sig");

    let account_id = OpenAiCodexProvider::extract_chatgpt_account_id(&jwt);
    assert_eq!(account_id.as_deref(), Some("org_test123"));
}
