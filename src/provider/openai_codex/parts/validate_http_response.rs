const RETRY_AFTER_MARKER: &str = "retry_after_seconds=";

fn retry_after_seconds(headers: &reqwest::header::HeaderMap) -> Option<u64> {
    headers
        .get(reqwest::header::RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim()
        .parse()
        .ok()
}

fn annotate_retry_after(message: String, seconds: Option<u64>) -> String {
    match seconds {
        Some(value) => format!("{message} [{RETRY_AFTER_MARKER}{value}]"),
        None => message,
    }
}

impl OpenAiCodexProvider {
    async fn validate_http_response(
        response: reqwest::Response,
        model: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let status = response.status();
        if !status.is_success() {
            let retry_after = retry_after_seconds(response.headers());
            let body = response.text().await.unwrap_or_default();
            let message = Self::format_openai_api_error(status, &body, model);
            anyhow::bail!(annotate_retry_after(message, retry_after));
        }
        Ok(Self::drive_responses_http_stream(response))
    }
}

#[cfg(test)]
#[path = "validate_http_response_tests.rs"]
mod validate_http_response_tests;
