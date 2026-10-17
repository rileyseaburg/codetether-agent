impl OpenAiCodexProvider {
    fn log_http_responses_request(backend: &str, body: &Value) {
        codetether_codex_transport::request_log::log_http_responses_request(backend, body)
    }
}
