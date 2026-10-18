impl OpenAiCodexProvider {
    #[cfg(test)]
    fn format_openai_api_error(status: StatusCode, body: &str, model: &str) -> String {
        codetether_codex_transport::format_openai_api_error(status, body, model)
    }
}
