impl OpenAiCodexProvider {
    fn drive_responses_http_stream(response: reqwest::Response) -> BoxStream<'static, StreamChunk> {
        Box::pin(async_stream::stream! {
            let mut parser = ResponsesSseParser::default();
            let mut byte_stream = response.bytes_stream();
            while let Some(result) = byte_stream.next().await {
                let chunks = match result {
                    Ok(bytes) => Self::parse_responses_sse_bytes(&mut parser, &bytes),
                    Err(error) => vec![StreamChunk::Error(error.to_string())],
                };
                for chunk in chunks {
                    yield chunk;
                }
            }
            for chunk in Self::finish_responses_sse_parser(&mut parser) {
                yield chunk;
            }
        })
    }
}
