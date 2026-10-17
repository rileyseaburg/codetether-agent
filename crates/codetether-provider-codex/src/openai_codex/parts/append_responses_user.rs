include!("user_image_content.rs");
impl OpenAiCodexProvider {
    fn append_responses_user(message: &Message, input: &mut Vec<Value>) {
        codetether_codex_protocol::CodexProtocol::append_responses_user(message, input)
    }
}
