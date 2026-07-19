impl OpenAiCodexProvider {
    async fn next_responses_sse_event<S>(
        stream: &mut S,
        parser: &mut ResponsesSseParser,
    ) -> Result<Option<Vec<StreamChunk>>, reqwest::Error>
    where
        S: futures::Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
    {
        loop {
            let Some(bytes) = stream.next().await.transpose()? else {
                let had_event =
                    !parser.line_buffer.is_empty() || !parser.event_data_lines.is_empty();
                let chunks = Self::finish_responses_sse_parser(parser);
                return Ok(had_event.then_some(chunks));
            };
            let (chunks, activity) =
                Self::parse_responses_sse_bytes_with_activity(parser, &bytes);
            if activity {
                return Ok(Some(chunks));
            }
        }
    }
}
