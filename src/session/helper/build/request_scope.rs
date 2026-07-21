//! Extraction of the actionable request from manager context envelopes.

/// Return only the CEO request when a manager prompt includes observed context.
pub(super) fn explicit(text: &str) -> &str {
    text.rsplit_once("CEO request:\n")
        .map_or(text, |(_, request)| request)
}

#[cfg(test)]
#[test]
fn ignores_context_paths_before_ceo_request() {
    let prompt = "Observed src/server.rs changed.\nCEO request:\nWho is working right now?";
    assert_eq!(explicit(prompt), "Who is working right now?");
}
