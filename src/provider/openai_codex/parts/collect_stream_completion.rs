impl OpenAiCodexProvider {
    async fn collect_stream_completion(
        mut stream: BoxStream<'static, StreamChunk>,
    ) -> Result<CompletionResponse> {
        let mut collector = CompletionCollector::default();
        while let Some(chunk) = stream.next().await {
            collector.accept(chunk)?;
        }
        collector.finish()
    }
}
