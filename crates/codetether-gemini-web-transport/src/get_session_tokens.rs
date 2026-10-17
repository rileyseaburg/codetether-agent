//! Gemini Web get session tokens.
use crate::*;
impl GeminiWebSession<'_> {
    /// GET the Gemini home page and extract the three tokens we need:
    ///   - `at_token`  XSRF token (key `thykhd` or legacy `SNlM0e`)
    ///   - `f_sid`     session ID (`FdrFJe`)
    ///   - `bl`        build label (`cfb2h`)
    ///
    /// Regexes are compiled once and reused across calls via `OnceLock`.
    ///
    /// # Errors
    /// Returns page transport, body-read, or required-token extraction errors.
    pub async fn get_session_tokens(&self) -> Result<SessionTokens> {
        static RE_NEW: OnceLock<Regex> = OnceLock::new();
        static RE_OLD: OnceLock<Regex> = OnceLock::new();
        static RE_BL: OnceLock<Regex> = OnceLock::new();
        static RE_SID: OnceLock<Regex> = OnceLock::new();

        let re_new = RE_NEW.get_or_init(|| Regex::new(r#""thykhd":"([^"]+)""#).unwrap());
        let re_old = RE_OLD.get_or_init(|| Regex::new(r#""SNlM0e":"([^"]+)""#).unwrap());
        let re_bl = RE_BL.get_or_init(|| Regex::new(r#""cfb2h":"([^"]+)""#).unwrap());
        let re_sid = RE_SID.get_or_init(|| Regex::new(r#""FdrFJe":"([^"]+)""#).unwrap());

        let cookie_hdr = self.cookie_header();
        let resp = self
            .client
            .get(GEMINI_ORIGIN)
            .header("Cookie", &cookie_hdr)
            .send()
            .await
            .context("Failed to fetch Gemini home page")?;

        // Extract the account prefix from the final URL after redirects.
        // Multi-account: redirects to /u/N/app  → prefix is "/u/N"
        // Primary account: redirects to /app    → prefix is ""
        let acct_prefix: String = {
            static RE_ACCT: OnceLock<Regex> = OnceLock::new();
            let re = RE_ACCT.get_or_init(|| Regex::new(r"(/u/\d+)/").unwrap());
            re.captures(resp.url().path())
                .map(|c| c[1].to_string())
                .unwrap_or_default()
        };
        tracing::debug!(acct_prefix = %acct_prefix, final_url = %resp.url(), "Gemini home page resolved");

        let html = resp
            .text()
            .await
            .context("Failed to read Gemini home page body")?;

        crate::parse_tokens::parse_tokens(&html, acct_prefix, re_new, re_old, re_bl, re_sid)
    }
}
