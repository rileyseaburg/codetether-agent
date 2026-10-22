pub(crate) mod fixture;
use codetether_context_policy::{
    pages::PageKind, state_header::prepend_state_header_with_corrections,
};
use codetether_provider_core::ContentPart;
use fixture::parent;

fn rendered(session: &fixture::Session) -> String {
    let mut messages = vec![];
    prepend_state_header_with_corrections(
        vec![],
        &session.pages,
        &session.messages,
        || 0,
        &session.metadata.constraint_corrections,
        &mut messages,
    );
    match &messages[0].content[0] {
        ContentPart::Text { text } => text.clone(),
        _ => panic!(),
    }
}
#[tokio::test]
async fn edited_prefix_rebases_links_and_preserves_pins_without_changing_parent() {
    let parent = parent();
    let child = crate::create(&parent, 2, "Continue").await.unwrap();
    assert_eq!(child.metadata.constraint_corrections[0].source_turn, 0);
    assert_eq!(child.metadata.constraint_corrections[0].replacement_turn, 1);
    assert!(rendered(&child).contains("Keep evidence."));
    assert!(rendered(&child).contains("Use blue."));
    assert!(!rendered(&child).contains("Use red."));
    assert_eq!(parent.metadata.constraint_corrections[0].source_turn, 100);
    let earlier = crate::create(&parent, 1, "Use blue.").await.unwrap();
    assert!(earlier.metadata.constraint_corrections.is_empty());
    assert!(rendered(&earlier).contains("Use red."));
}
#[tokio::test]
async fn detached_tail_only_keeps_links_whose_sources_are_retained() {
    let parent = parent();
    let full = crate::detached::create(&parent, || 3).await.unwrap();
    assert!(!rendered(&full).contains("Use red."));
    let tail = crate::detached::create(&parent, || 2).await.unwrap();
    assert!(tail.metadata.constraint_corrections.is_empty());
    assert!(rendered(&tail).contains("Use blue."));
    assert_eq!(tail.pages[0], PageKind::Constraint);
    let mut shifted = fixture::parent();
    shifted.messages.insert(0, fixture::message("earlier"));
    shifted.pages.insert(0, PageKind::Conversation);
    shifted.metadata.constraint_corrections[0].source_turn += 1;
    shifted.metadata.constraint_corrections[0].replacement_turn += 1;
    let shifted_tail = crate::detached::create(&shifted, || 3).await.unwrap();
    assert_eq!(
        shifted_tail.metadata.constraint_corrections[0].source_turn,
        0
    );
    assert!(!rendered(&shifted_tail).contains("Use red."));
    assert!(rendered(&shifted_tail).contains("Keep evidence."));
}
#[test]
fn whole_replacement_links_rebase_or_restore_the_original_before_the_correction() {
    let mut parent = parent();
    parent.pages[0] = PageKind::SupersededConstraint { by_turn: 101 };
    let (pages, _) = crate::constraints::retained(&parent, 0, 2);
    assert_eq!(pages[0], PageKind::SupersededConstraint { by_turn: 1 });
    let (pages, _) = crate::constraints::retained(&parent, 0, 1);
    assert_eq!(pages[0], PageKind::Constraint);
}
