#[test]
fn extracts_chatgpt_account_id_from_organizations_claim() {
    let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"none","typ":"JWT"}"#);
    let payload = URL_SAFE_NO_PAD.encode(r#"{"organizations":[{"id":"org_from_list"}]}"#);
    let jwt = format!("{header}.{payload}.sig");

    let account_id = OpenAiCodexProvider::extract_chatgpt_account_id(&jwt);
    assert_eq!(account_id.as_deref(), Some("org_from_list"));
}
