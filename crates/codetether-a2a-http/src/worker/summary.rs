//! Bound and normalize remote response text for diagnostics.

pub fn summarize_response_body(body: &str) -> String {
    const MAX_BODY_CHARS: usize = 512;
    let mut summary = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if summary.chars().count() > MAX_BODY_CHARS {
        summary = summary.chars().take(MAX_BODY_CHARS).collect::<String>();
        summary.push_str("...");
    }
    summary
}
