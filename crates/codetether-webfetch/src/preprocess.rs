//! preprocess for web content fetching.
use super::*;
impl WebFetchTool {
    pub(crate) fn preprocess_html(&self, html: &str) -> String {
        // Remove the highest-noise/highest-token sections first.
        // Many docs sites embed large JS bundles and assistant widgets inside
        // <script> tags; we do not want that in agent context.
        let mut s = html.to_string();

        // Strip common "noise" blocks that otherwise dominate the output.
        // (Rust regex has no backreferences, so we do them explicitly.)
        for re in [
            &*RE_STRIP_SCRIPT,
            &*RE_STRIP_STYLE,
            &*RE_STRIP_NOSCRIPT,
            &*RE_STRIP_SVG,
            &*RE_STRIP_CANVAS,
            &*RE_STRIP_IFRAME,
        ] {
            s = re.replace_all(&s, "").to_string();
        }

        s = RE_STRIP_HTML_COMMENTS.replace_all(&s, "").to_string();
        s
    }
}
