//! Decode persisted tool feedback without mistaking raw runtime output for an envelope.
pub(super) fn decode(content: &str, success: Option<bool>) -> Option<(bool, &str)> {
    let (reported_failure, output) =
        if success.is_none() && content.starts_with("Tool call feedback\n") {
            let (header, output) = content
                .split_once("\n\nOutput:\n")
                .unwrap_or((content, content));
            (
                header.lines().any(|line| line == "- status: failure"),
                output,
            )
        } else {
            (false, content)
        };
    if output
        .trim_start()
        .trim_start_matches("Error: ")
        .starts_with("REPEATED_EDIT_BLOCKED:")
    {
        return None;
    }

    Some((success.map_or(reported_failure, |success| !success), output))
}
