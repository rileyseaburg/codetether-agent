use super::summary::SessionSummary;

pub(in crate::session) fn merge_summary(
    mut native: SessionSummary,
    codex: SessionSummary,
) -> SessionSummary {
    if native.updated_at < codex.updated_at {
        return codex;
    }
    let title_is_valid = match native.title.as_deref() {
        Some(title) => crate::session::title::is_title_candidate(title),
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
    use crate::session::title::is_title_candidate;

    #[test]
    fn injected_title_is_not_a_candidate() {
        assert!(!is_title_candidate("# AGENTS.md instructions for /tmp/x"));
        assert!(is_title_candidate("see the elevon door hanger work"));
    }
}
