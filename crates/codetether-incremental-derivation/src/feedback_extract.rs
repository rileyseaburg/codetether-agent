//! Recognize legacy structured denials without promoting their authority.
pub(super) fn extract(content: &str) -> Option<String> {
    let (header, body) = content.split_once("\n\nOutput:\n")?;
    if !header.starts_with("Tool call feedback\n")
        || !header.lines().any(|line| line == "- status: failure")
    {
        return None;
    }
    let body: serde_json::Value = serde_json::from_str(body).ok()?;
    let error = body.get("error")?;
    (error.get("code")?.as_str()? == "TOOL_APPROVAL_DENIED")
        .then(|| error.get("message")?.as_str().map(str::to_owned))
        .flatten()
}
