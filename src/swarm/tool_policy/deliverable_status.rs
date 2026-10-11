//! Detect explicit non-completion statuses in sub-agent summaries.

/// Missing status metadata is not an execution failure; empty output still is.
pub(crate) fn error(output: &str) -> Option<String> {
    let Some(status) = output.lines().rev().find(|line| !line.trim().is_empty()) else {
        return Some("Sub-agent produced no deliverable output".into());
    };
    let status = status
        .trim()
        .trim_matches(|character: char| matches!(character, '*' | '`' | '.' | '!'))
        .to_ascii_lowercase();
    let footer = status.rsplit_once("status:");
    let status = footer.map(|(_, value)| value.trim()).unwrap_or(&status);
    match status {
        "blocked" => Some("Sub-agent reported the deliverable as blocked".into()),
        "pending" => Some("Sub-agent reported the deliverable as pending".into()),
        "completed" => None,
        _ if footer.is_some() => {
            Some("Sub-agent reported an unrecognized terminal deliverable status".into())
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "deliverable_status_tests.rs"]
mod tests;
