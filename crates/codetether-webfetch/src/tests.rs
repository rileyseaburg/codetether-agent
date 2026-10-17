//! tests for web content fetching.

use super::*;

#[test]
fn html_to_markdown_strips_script_content() {
    let tool = WebFetchTool::new();
    let html = r#"<html><head><title>x</title></head><body>
<h1>Hello</h1>
<script>window.__assistant_state = { open: true }; function big(){ return 1; }</script>
<p>World</p>
</body></html>"#;

    let md = tool.html_to_markdown(html);
    assert!(md.contains("Hello"));
    assert!(md.contains("World"));
    assert!(!md.contains("__assistant_state"));
    assert!(!md.contains("function big"));
}
