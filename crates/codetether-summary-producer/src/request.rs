//! Preserve the summary request text and transcript projection.
use codetether_provider_core::Message;
pub(super) fn format(
    slice: &[Message],
    target_tokens: usize,
    provenance: &codetether_context_policy::summary_index::SummaryProvenance,
) -> String {
    let context = codetether_context_policy::transcript::messages_to_rlm_context(slice);
    let policy = codetether_context_policy::summary_index::SUMMARY_POLICY;
    let version = codetether_context_policy::summary_index::SUMMARY_POLICY_VERSION;
    let input = format!(
        "Summary policy {version}: {policy}\nSource SHA-256: {}\nRequested model: {:?}\nSummarise the following transcript window (~{} messages, target ≤ {} tokens).\n\n{context}",
        provenance.source_sha256,
        provenance.requested_model,
        slice.len(),
        target_tokens,
    );
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use codetether_context_policy::summary_index::{SUMMARY_POLICY_VERSION, SummaryProvenance};
    use codetether_provider_core::{ContentPart, Role};
    #[test]
    fn background_input_identity_includes_source_policy_and_requested_model() {
        let messages = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "keep API".into(),
            }],
        }];
        let provenance =
            SummaryProvenance::new(&messages, "background-or-fallback", Some("model-a")).unwrap();
        let input = format(&messages, 100, &provenance);
        assert!(input.contains(SUMMARY_POLICY_VERSION));
        assert!(input.contains(&provenance.source_sha256));
        assert!(input.contains("Distinguish attempted tools"));
        let another_model =
            SummaryProvenance::new(&messages, "background-or-fallback", Some("model-b")).unwrap();
        assert_ne!(input, format(&messages, 100, &another_model));
        let mut changed = messages;
        changed[0].role = Role::Assistant;
        let changed_provenance =
            SummaryProvenance::new(&changed, "background-or-fallback", Some("model-a")).unwrap();
        assert_ne!(provenance.source_sha256, changed_provenance.source_sha256);
        assert_ne!(input, format(&changed, 100, &changed_provenance));
    }
}
