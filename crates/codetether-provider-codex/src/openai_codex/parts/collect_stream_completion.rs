impl OpenAiCodexProvider {
    async fn collect_stream_completion(
        stream: BoxStream<'static, StreamChunk>,
    ) -> Result<CompletionResponse> {
        codetether_codex_transport::collect_stream_completion(stream).await
    }
}
