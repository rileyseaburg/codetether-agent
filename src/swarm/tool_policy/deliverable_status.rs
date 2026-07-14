//! Detect explicit non-completion statuses in sub-agent summaries.

pub(crate) fn error(output: &str) -> Option<String> {
    let Some(status) = output.lines().rev().find(|line| !line.trim().is_empty()) else {
        return Some("Sub-agent produced no deliverable status".into());
    };
    let status = status
        .trim()
        .trim_matches(|character: char| matches!(character, '*' | '`' | '.' | '!'))
        .to_ascii_lowercase();
    let status = status
        .rsplit_once("status:")
        .map(|(_, value)| value.trim())
        .unwrap_or(&status);
    match status {
        "blocked" => Some("Sub-agent reported the deliverable as blocked".into()),
        "pending" => Some("Sub-agent reported the deliverable as pending".into()),
        "completed" => None,
        _ => Some("Sub-agent omitted the required terminal deliverable status".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::error;

    #[test]
    fn rejects_explicit_non_completion() {
        assert!(error("evidence\nSTATUS: blocked").is_some());
        assert!(error("evidence\nSTATUS: pending").is_some());
        assert!(error("evidence\nSTATUS: completed").is_none());
        assert!(error("evidence. STATUS: completed").is_none());
        assert!(error("evidence only").is_some());
    }
}
