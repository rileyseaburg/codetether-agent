impl OpenAiCodexProvider {
    #[cfg(test)]
    fn parse_responses_sse_bytes(
        parser: &mut ResponsesSseParser,
        bytes: &[u8],
    ) -> Vec<StreamChunk> {
        codetether_codex_protocol::CodexProtocol::parse_responses_sse_bytes(parser, bytes)
    }

    #[cfg(test)]
    fn parse_responses_sse_bytes_with_activity(
        parser: &mut ResponsesSseParser,
        bytes: &[u8],
    ) -> (Vec<StreamChunk>, bool) {
        codetether_codex_protocol::CodexProtocol::parse_responses_sse_bytes_with_activity(
            parser, bytes,
        )
    }
}
