//! markdown for web content fetching.
use super::*;
impl WebFetchTool {
    pub(crate) fn html_to_markdown(&self, html: &str) -> String {
        let html = self.preprocess_html(html);
        let mut result = html;
        let patterns = [
            (r"<h1[^>]*>(.*?)</h1>", "# $1\n"),
            (r"<h2[^>]*>(.*?)</h2>", "## $1\n"),
            (r"<h3[^>]*>(.*?)</h3>", "### $1\n"),
            (r"<p[^>]*>(.*?)</p>", "$1\n\n"),
            (r"<(strong|b)[^>]*>(.*?)</\1>", "**$2**"),
            (r"<(em|i)[^>]*>(.*?)</\1>", "*$2*"),
            (r"<code[^>]*>(.*?)</code>", "`$1`"),
            (r"<li[^>]*>(.*?)</li>", "- $1\n"),
        ];
        for (pat, rep) in patterns {
            if let Ok(re) = regex::Regex::new(pat) {
                result = re.replace_all(&result, rep).to_string();
            }
        }
        result = regex::Regex::new(r"<[^>]+>")
            .unwrap()
            .replace_all(&result, "")
            .to_string();
        result
            .replace("&nbsp;", " ")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
    }
}
