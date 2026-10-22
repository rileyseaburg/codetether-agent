use codetether_agent::session::Session;
use codetether_context_policy::summary_index::{
    Granularity, SummaryNode, SummaryProvenance, SummaryRange,
};

pub async fn seed(session: &mut Session) -> anyhow::Result<()> {
    let end = session.messages.len();
    for (start, end, label) in [
        (end - 8, end - 4, "stable summary"),
        (end - 4, end, "summary to invalidate"),
    ] {
        let range = SummaryRange::new(start, end).unwrap();
        let node = SummaryNode {
            content: label.into(),
            target_tokens: 100,
            granularity: Granularity::Turn,
            generation: 0,
            provenance: Some(SummaryProvenance::new(
                &session.messages[start..end],
                "deterministic-fixture",
                Some("fixture-model"),
            )?),
        };
        codetether_session_store::summaries::put_snapshot(session, range, &node).await?;
        session.summary_index.insert(range, node);
    }
    Ok(())
}
pub fn verify(session: &Session, phase: usize) {
    let entries: Vec<_> = session.summary_index.entries().collect();
    let stable = entries
        .iter()
        .find(|(_, node)| node.content == "stable summary")
        .unwrap_or_else(|| panic!("phase {phase}: unchanged range missing from {entries:?}"));
    assert!(stable.1.valid_for(&session.messages, *stable.0));
    assert_eq!(
        entries
            .iter()
            .any(|(_, node)| node.content == "summary to invalidate"),
        phase == 0
    );
    let mut old_policy = stable.1.clone();
    old_policy.provenance.as_mut().unwrap().policy_version = "obsolete-policy".into();
    assert!(!old_policy.valid_for(&session.messages, *stable.0));
}
pub fn invalidate_one(session: &mut Session) {
    let (range, node) = session
        .summary_index
        .entries()
        .find(|(_, node)| node.content == "summary to invalidate")
        .map(|(range, node)| (*range, node.clone()))
        .unwrap();
    session.messages[range.start] = super::fixtures::text(
        codetether_provider_core::Role::User,
        "Edited archived source",
    );
    assert!(!node.valid_for(&session.messages, range));
}
