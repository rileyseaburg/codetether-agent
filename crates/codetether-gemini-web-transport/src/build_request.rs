//! Build an authenticated Gemini StreamGenerate request.
use crate::*;
impl GeminiWebSession<'_> {
    /// Build a `RequestBuilder` for the StreamGenerate endpoint.
    ///
    /// # Errors
    /// Returns token-refresh or endpoint URL construction failures.
    pub async fn build_request(
        &self,
        prompt: &str,
        model: &str,
    ) -> Result<reqwest::RequestBuilder> {
        let tokens = self
            .get_or_refresh_tokens()
            .await
            .context("Failed to obtain Gemini session tokens")?;

        let cookie_hdr = self.cookie_header();
        let freq = GeminiWebProtocol::build_freq(prompt);
        let mode_id = GeminiWebProtocol::mode_id(model);

        let ext_header = crate::request_metadata::extension_header(mode_id);

        let reqid = crate::request_metadata::request_id();

        let endpoint = format!(
            "https://gemini.google.com{}{}",
            tokens.acct_prefix, GEMINI_STREAM_PATH
        );
        tracing::debug!(endpoint = %endpoint, "Gemini StreamGenerate endpoint");
        let url = crate::request_url::request_url(&endpoint, &tokens, &reqid)?;

        Ok(self
            .client
            .post(url)
            .header("Cookie", cookie_hdr)
            .header("X-Same-Domain", "1")
            .header("Origin", GEMINI_ORIGIN)
            .header("Referer", format!("{}/app", GEMINI_ORIGIN))
            .header("Accept", "*/*")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("x-goog-ext-525001261-jspb", ext_header)
            .form(&[("f.req", freq), ("at", tokens.at_token)]))
    }
}
