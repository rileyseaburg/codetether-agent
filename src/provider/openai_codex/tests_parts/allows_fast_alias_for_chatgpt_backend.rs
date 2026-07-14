#[test]
fn allows_fast_alias_for_chatgpt_backend() {
    let provider = OpenAiCodexProvider::new();
    // gpt-5.5-fast resolves to gpt-5.5 which is the only supported model.
    provider
        .validate_model_for_backend("gpt-5.5-fast:high")
        .expect("chatgpt backend should allow GPT-5.5 fast alias");
}
