//! result for web content fetching.
use super::*;
pub(super) fn render(content: String, p: &params::Params, github_auth: bool) -> Result<ToolResult> {
    let (out, truncated) = truncate::truncate(content, p.max_chars);
    Ok(ToolResult::success(out)
        .with_metadata("url", json!(p.url))
        .with_metadata("format", json!(p.format))
        .with_metadata("github_auth", json!(github_auth))
        .with_metadata("truncated", json!(truncated))
        .with_metadata("max_chars", json!(p.max_chars)))
}
