impl OpenAiCodexProvider {
    fn finish_responses_sse_parser(parser: &mut ResponsesSseParser) -> Vec<StreamChunk> {
        codetether_codex_protocol::CodexProtocol::finish_responses_sse_parser(parser)
    }
}
