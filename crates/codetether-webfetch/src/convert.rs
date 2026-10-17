//! convert for web content fetching.
use super::*;
impl WebFetchTool {
    pub(crate) fn convert(&self, ct: &str, body: String, format: &str) -> String {
        match format {
            "html" => body,
            "text" => {
                if self.is_html(&ct, &body) {
                    self.html_to_text(&body)
                } else {
                    body
                }
            }
            _ => {
                if self.is_html(&ct, &body) {
                    self.html_to_markdown(&body)
                } else {
                    body
                }
            }
        }
    }
}
