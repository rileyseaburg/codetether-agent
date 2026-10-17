impl OpenAiCodexProvider {
    fn parse_responses_event(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
    ) {
        codetether_codex_protocol::CodexProtocol::parse_responses_event(parser, event, chunks)
    }
}
