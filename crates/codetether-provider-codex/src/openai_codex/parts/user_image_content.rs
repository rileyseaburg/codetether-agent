use codetether_codex_protocol::UserImageFormat;
impl OpenAiCodexProvider {
    fn user_has_images(message: &Message) -> bool {
        codetether_codex_protocol::CodexProtocol::user_has_images(message)
    }

    fn user_image_content(message: &Message, format: UserImageFormat) -> Vec<Value> {
        codetether_codex_protocol::CodexProtocol::user_image_content(message, format)
    }
}
