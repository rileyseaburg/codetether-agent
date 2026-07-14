impl OpenAiCodexProvider {
    fn finish_responses_sse_parser(parser: &mut ResponsesSseParser) -> Vec<StreamChunk> {
        let mut chunks = Vec::new();

        if !parser.line_buffer.is_empty() {
            let mut line = std::mem::take(&mut parser.line_buffer);
            if line.ends_with('\r') {
                line.pop();
            }
            if let Some(data) = line.strip_prefix("data:") {
                let data = data.strip_prefix(' ').unwrap_or(data);
                parser.event_data_lines.push(data.to_string());
            }
        }

        if !parser.event_data_lines.is_empty() {
            let data = parser.event_data_lines.join("\n");
            parser.event_data_lines.clear();
            Self::parse_responses_sse_event(parser, &data, &mut chunks);
        }

        chunks
    }
}
