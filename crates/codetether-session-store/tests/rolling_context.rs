//! Exercise summary retention through the actual bounded append path.
use codetether_context_policy::{
    pages::PageKind,
    summary_index::{Granularity, SummaryIndex, SummaryNode, SummaryRange},
};
use codetether_provider_core::{Message, Role};
use codetether_session_store::{
    State,
    window::{WINDOW, WorkingSet, append},
};
use codetether_tracked_vec::TrackedVec;

#[test]
fn many_turn_appends_preserve_summaries_with_correct_coordinates() {
    let state = State::default();
    {
        let mut checkpoint = state.0.lock().unwrap();
        checkpoint.id = "long-task".into();
        checkpoint.revision = 1;
    }
    let message = Message {
        role: Role::User,
        content: vec![],
    };
    let mut messages: TrackedVec<_> = vec![message.clone(); WINDOW].into();
    let mut pages: TrackedVec<_> = vec![PageKind::Conversation; WINDOW].into();
    let mut tools: TrackedVec<()> = Default::default();
    let mut summaries = SummaryIndex::new();
    summaries.insert(
        SummaryRange::new(100, 900).unwrap(),
        SummaryNode {
            content: "completed work and remaining steps".into(),
            target_tokens: 64,
            granularity: Granularity::Phase,
            generation: 0,
            provenance: Some(
                codetether_context_policy::summary_index::SummaryProvenance::new(
                    &messages[100..900],
                    "fixture",
                    None,
                )
                .unwrap(),
            ),
        },
    );
    for removed in 1..=101 {
        messages.clean();
        pages.clean();
        append(
            WorkingSet {
                id: "long-task",
                state: &state,
                messages: &mut messages,
                pages: &mut pages,
                tools: &mut tools,
                summaries: &mut summaries,
            },
            message.clone(),
        );
        assert_eq!(messages.len(), WINDOW);
        assert_eq!(pages.len(), WINDOW);
        assert_eq!(state.0.lock().unwrap().message_start, removed);
        if removed <= 100 {
            let range = SummaryRange::new(100 - removed, 900 - removed).unwrap();
            assert!(summaries.get(range).unwrap().valid_for(&messages, range));
            assert_eq!(
                summaries.get(range).unwrap().content,
                "completed work and remaining steps"
            );
        } else {
            assert!(
                summaries.is_empty(),
                "a partially evicted summary is no longer valid"
            );
        }
    }
}
