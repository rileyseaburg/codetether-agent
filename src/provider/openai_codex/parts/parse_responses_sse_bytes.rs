impl OpenAiCodexProvider {
    #[cfg(test)]
    fn parse_responses_sse_bytes(
        parser: &mut ResponsesSseParser,
        bytes: &[u8],
    ) -> Vec<StreamChunk> {
        Self::parse_responses_sse_bytes_with_activity(parser, bytes).0
    }

    fn parse_responses_sse_bytes_with_activity(
        parser: &mut ResponsesSseParser,
        bytes: &[u8],
    ) -> (Vec<StreamChunk>, bool) {
        parser.line_buffer.push_str(&String::from_utf8_lossy(bytes));
        let mut chunks = Vec::new();
        let mut activity = false;

        while let Some(line_end) = parser.line_buffer.find('\n') {
            let mut line = parser.line_buffer[..line_end].to_string();
            parser.line_buffer.drain(..=line_end);

            if line.ends_with('\r') {
                line.pop();
            }

            if line.is_empty() {
                if !parser.event_data_lines.is_empty() {
                    activity = true;
                    let data = parser.event_data_lines.join("\n");
                    parser.event_data_lines.clear();
                    Self::parse_responses_sse_event(parser, &data, &mut chunks);
                }
                continue;
            }

            if let Some(data) = line.strip_prefix("data:") {
                let data = data.strip_prefix(' ').unwrap_or(data);
                parser.event_data_lines.push(data.to_string());
            }
        }

        (chunks, activity)
    }
}
