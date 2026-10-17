//! detect for web content fetching.
use super::*;
impl WebFetchTool {
    pub(crate) fn is_html(&self, ct: &str, body: &str) -> bool {
        ct.contains("text/html") || body.trim().starts_with('<')
    }
}
