//! text for web content fetching.
use super::*;
impl WebFetchTool {
    pub(crate) fn html_to_text(&self, html: &str) -> String {
        let md = self.html_to_markdown(html);
        md.replace("**", "")
            .replace("*", "")
            .replace("`", "")
            .replace("# ", "")
    }
}
