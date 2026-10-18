use super::summary::SessionSummary;

/// Select the newest summary, repairing an invalid native title on native wins.
/// # Arguments
/// * `native` — Summary from the native session store.
/// * `codex` — Summary imported from Codex history.
/// # Returns
/// The newer summary; ties retain native data with invalid-title repair.
/// # Examples
/// ```no_run
/// # fn example(native: codetether_session_metadata::SessionSummary,
/// # codex: codetether_session_metadata::SessionSummary) {
/// let latest = native.updated_at.max(codex.updated_at);
/// let merged = codetether_session_listing::merge_summary(native, codex);
/// assert_eq!(merged.updated_at, latest);
/// # }
/// ```
pub fn merge_summary(mut native: SessionSummary, codex: SessionSummary) -> SessionSummary {
    if native.updated_at < codex.updated_at {
        return codex;
    }
    let title_is_valid = match native.title.as_deref() {
        Some(title) => codetether_session_title::is_title_candidate(title),
        None => true,
    };
    if title_is_valid {
        return native;
    }
    native.title = codex.title;
    native
}

#[cfg(test)]
mod tests {
    use codetether_session_title::is_title_candidate;

    #[test]
    fn injected_title_is_not_a_candidate() {
        assert!(!is_title_candidate("# AGENTS.md instructions for /tmp/x"));
        assert!(is_title_candidate("see the elevon door hanger work"));
    }
}
