#[test]
fn allows_pro_model_for_api_key_backend() {
    let provider = OpenAiCodexProvider::from_api_key("test-key".to_string());
    provider
        .validate_model_for_backend("gpt-5.4-pro")
        .expect("api key backend should allow pro model");
}
