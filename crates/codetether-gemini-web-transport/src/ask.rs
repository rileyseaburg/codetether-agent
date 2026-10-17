//! Bounded Gemini HTTP and empty-response retries.
use crate::*;
impl GeminiWebSession<'_> {
    /// Core blocking request: POST and collect the complete response.
    ///
    /// # Errors
    /// Returns send/read failures or the existing error after one retry.
    pub async fn ask(&self, prompt: &str, model: &str) -> Result<String> {
        for attempt in 0..=1 {
            let resp = self.send_request(prompt, model).await?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                if attempt == 0 {
                    tracing::warn!(
                        status = %status,
                        body_prefix = %codetether_util::truncate::truncate_bytes_safe(&body, 200),
                        "Gemini request failed; invalidating cached tokens and retrying once"
                    );
                    self.invalidate_tokens().await;
                    continue;
                }
                anyhow::bail!(
                    "Gemini StreamGenerate returned HTTP {}: {}",
                    status,
                    codetether_util::truncate::truncate_bytes_safe(&body, 500)
                );
            }

            let body = crate::read_response::read_response(resp).await?;
            let text = GeminiWebProtocol::extract_text(&body);
            if text.is_empty() {
                let protocol_code = GeminiWebProtocol::extract_protocol_error_code(&body);
                if attempt == 0 {
                    tracing::warn!(
                        body_prefix = %codetether_util::truncate::truncate_bytes_safe(&body, 200),
                        "Gemini response had no parseable text; invalidating cached tokens and retrying once"
                    );
                    self.invalidate_tokens().await;
                    continue;
                }
                if let Some(code) = protocol_code {
                    anyhow::bail!(GeminiWebProtocol::format_protocol_error(code, model, &body));
                }
                anyhow::bail!(
                    "No text found in Gemini response for model `{}`. Payload snippet: {}",
                    model,
                    GeminiWebProtocol::compact_body_snippet(&body, 240)
                );
            }
            return Ok(text);
        }

        anyhow::bail!("Gemini request retry exhausted without a successful response")
    }
}
