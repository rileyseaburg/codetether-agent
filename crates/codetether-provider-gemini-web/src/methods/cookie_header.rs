//! Provider cookie header.
use crate::*;
impl GeminiWebProvider {
    /// Convert a Netscape cookies.txt (7 tab-separated columns) into a
    /// `Cookie: name=value; ...` header string.
    ///
    /// Standard Netscape format columns:
    ///   0: domain  1: flag  2: path  3: secure  4: expiration  5: name  6: value
    ///
    /// If only 2 columns are present (simple `name\tvalue` format), those are
    /// used directly so the function remains backward-compatible.
    pub(crate) fn cookie_header(&self) -> String {
        codetether_gemini_web_transport::GeminiWebSession::new(
            &self.client,
            &self.cookies,
            &self.token_cache,
        )
        .cookie_header()
    }
}
