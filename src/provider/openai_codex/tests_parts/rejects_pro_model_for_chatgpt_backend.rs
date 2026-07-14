#[test]
fn rejects_pro_model_for_chatgpt_backend() {
    let provider = OpenAiCodexProvider::new();
    let err = provider
        .validate_model_for_backend("gpt-5.4-pro")
        .expect_err("chatgpt backend should reject unsupported model");
    assert!(
        err.to_string()
            .contains("not supported when using Codex with a ChatGPT account")
    );
}
