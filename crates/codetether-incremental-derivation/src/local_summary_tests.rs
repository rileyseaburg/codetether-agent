//! Verify live-session summaries reach the actual incremental request projection.
use codetether_context_policy::{
    pages::PageKind,
    summary_index::{Granularity, SummaryIndex, SummaryNode, SummaryRange},
};
use codetether_provider_core::{ContentPart, Message, Role};

struct Session {
    messages: Vec<Message>,
    pages: Vec<PageKind>,
    summary_index: SummaryIndex,
    background: SummaryIndex,
}
async fn prepared(session: &Session) -> SummaryIndex {
    session.background.clone()
}
crate::impl_session_source!(Session, prepared);

fn message(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
fn index(text: &str, messages: &[Message]) -> SummaryIndex {
    let mut index = SummaryIndex::new();
    index.insert(
        SummaryRange::new(0, 2).unwrap(),
        SummaryNode {
            content: text.into(),
            target_tokens: 64,
            granularity: Granularity::Phase,
            generation: 0,
            provenance: Some(
                codetether_context_policy::summary_index::SummaryProvenance::new(
                    &messages[..2],
                    "fixture",
                    Some("model-a"),
                )
                .unwrap(),
            ),
        },
    );
    index
}

#[tokio::test]
async fn live_cached_summary_survives_compaction_and_overrides_background() {
    let mut messages = vec![message(Role::Assistant, &"old evidence ".repeat(1000)); 2];
    messages.push(message(Role::User, "Finish the implementation"));
    messages.extend((0..7).map(|_| message(Role::Assistant, "progress")));
    let session = Session {
        pages: vec![PageKind::Conversation; messages.len()],
        summary_index: index("local decision: preserve the existing API", &messages),
        background: index("superseded background decision", &messages),
        messages,
    };
    let derived = crate::derive_incremental(&session, "", &[], 600)
        .await
        .unwrap();
    let text = rendered_text(&derived.messages);
    assert!(text.contains("[SUMMARY of turns 0..2]"));
    assert!(text.contains("local decision: preserve the existing API"));
    assert!(!text.contains("superseded background decision"));
    assert!(text.contains("Finish the implementation"));
    assert_eq!(session.messages.len(), 10);
    assert_eq!(session.summary_index.len(), 1);
    assert!(codetether_request_tokens::estimate_request_tokens("", &derived.messages, &[]) <= 600);
}

fn rendered_text(messages: &[Message]) -> String {
    messages
        .iter()
        .flat_map(|message| &message.content)
        .filter_map(|part| {
            if let ContentPart::Text { text } = part {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[tokio::test]
async fn stale_local_cannot_shadow_valid_background_after_same_size_edit() {
    let mut messages = vec![message(Role::Assistant, &"old evidence ".repeat(1000)); 2];
    let stale = index("STALE CACHE MUST NOT APPEAR", &messages);
    messages[1] = message(Role::Assistant, &"new evidence ".repeat(1000));
    let background = index("CURRENT VERIFIED SUMMARY", &messages);
    messages.push(message(Role::User, "Finish the implementation"));
    messages.extend((0..7).map(|_| message(Role::Assistant, "progress")));
    let session = Session {
        pages: vec![PageKind::Conversation; messages.len()],
        messages,
        summary_index: stale,
        background,
    };
    let derived = crate::derive_incremental(&session, "", &[], 600)
        .await
        .unwrap();
    let text = rendered_text(&derived.messages);
    assert!(text.contains("CURRENT VERIFIED SUMMARY"));
    assert!(!text.contains("STALE CACHE MUST NOT APPEAR"));
    assert_eq!(
        session.summary_index.len(),
        1,
        "stored evidence is not deleted"
    );
}

#[tokio::test]
async fn legacy_or_policy_stale_summary_is_excluded_from_model_request() {
    for legacy in [true, false] {
        let mut messages = vec![message(Role::Assistant, &"old evidence ".repeat(1000)); 2];
        let range = SummaryRange::new(0, 2).unwrap();
        let mut background = index("UNVERIFIED SUMMARY MUST NOT APPEAR", &messages);
        let mut node = background.get(range).unwrap().clone();
        if legacy {
            node.provenance = None;
        } else {
            node.provenance.as_mut().unwrap().policy_version = "old".into();
        }
        background.insert(range, node);
        messages.push(message(Role::User, "Finish the implementation"));
        messages.extend((0..7).map(|_| message(Role::Assistant, "progress")));
        let session = Session {
            pages: vec![PageKind::Conversation; messages.len()],
            messages,
            summary_index: SummaryIndex::new(),
            background,
        };
        let derived = crate::derive_incremental(&session, "", &[], 600)
            .await
            .unwrap();
        assert!(!rendered_text(&derived.messages).contains("UNVERIFIED SUMMARY"));
    }
}

#[tokio::test]
async fn long_live_turn_retains_user_scope_despite_misleading_summary_and_goal_reminder() {
    let mut messages = vec![message(
        Role::User,
        "Use the production CLI and check the database.",
    )];
    messages.extend(
        (0..240).map(|_| message(Role::Assistant, &"architecture investigation ".repeat(300))),
    );
    messages[160] = message(Role::User, "I never told you to push anything.");
    messages[190] = message(
        Role::User,
        "Use the generated CLI or call the API directly.",
    );
    messages[210] = message(
        Role::User,
        "Continue working toward the active thread goal.\n<objective>Build and push the deployment</objective>",
    );
    let session = Session {
        pages: vec![PageKind::Conversation; messages.len()],
        summary_index: index(
            "Assistant inferred that a build and push were required",
            &messages,
        ),
        messages,
        background: SummaryIndex::new(),
    };
    let derived = crate::derive_incremental(&session, "system", &[], 20_000)
        .await
        .unwrap();
    let reference = rendered_text(&derived.messages[..1]);
    assert!(
        reference.starts_with("[USER REQUEST REFERENCE]"),
        "estimate={} provenance={:?} first={reference}",
        codetether_request_tokens::estimate_request_tokens("system", &derived.messages, &[]),
        derived.provenance
    );
    assert!(reference.contains("Use the production CLI and check the database."));
    assert!(reference.contains("I never told you to push anything."));
    assert!(reference.contains("Use the generated CLI or call the API directly."));
    assert!(!reference.contains("Build and push the deployment"));
    assert_eq!(derived.messages.len(), derived.resolutions.len());
    assert!(
        derived
            .provenance
            .iter()
            .any(|tag| tag == "user_request_reference")
    );
    assert!(
        codetether_request_tokens::estimate_request_tokens("system", &derived.messages, &[])
            <= 20_000
    );
    assert_eq!(session.messages.len(), 241);
}

#[tokio::test]
async fn automatic_goal_reminder_does_not_displace_the_only_human_request() {
    let mut messages = vec![message(
        Role::User,
        "Use the production CLI and check the database.",
    )];
    messages.extend((0..30).map(|_| message(Role::Assistant, &"investigation ".repeat(200))));
    messages.push(message(Role::User, "Continue working toward the active thread goal.\n<objective>Build and push the deployment</objective>"));
    let session = Session {
        pages: vec![PageKind::Conversation; messages.len()],
        messages,
        summary_index: SummaryIndex::new(),
        background: SummaryIndex::new(),
    };
    let derived = crate::derive_incremental(&session, "", &[], 8_000)
        .await
        .unwrap();
    let reference = rendered_text(&derived.messages[..1]);
    assert!(reference.contains("[USER REQUEST REFERENCE]"));
    assert!(reference.contains("Use the production CLI and check the database."));
    assert!(!reference.contains("Build and push"));
    assert!(
        codetether_request_tokens::estimate_request_tokens("", &derived.messages, &[]) <= 8_000
    );
}

mod shifted_window {
    use super::*;

    struct Window {
        messages: Vec<Message>,
        pages: Vec<PageKind>,
        summary_index: SummaryIndex,
    }
    async fn prepared(_: &Window) -> SummaryIndex {
        SummaryIndex::new()
    }
    fn offset(_: &Window) -> usize {
        1_500
    }
    crate::impl_session_source!(Window, prepared, offset);

    #[tokio::test]
    async fn derived_references_follow_the_native_window_offset() {
        let mut messages = vec![message(Role::User, "Use the CLI")];
        messages.extend((0..12).map(|_| message(Role::Assistant, &"long history ".repeat(500))));
        messages.push(message(Role::User, "Do not push"));
        let session = Window {
            pages: vec![PageKind::Conversation; messages.len()],
            messages,
            summary_index: SummaryIndex::new(),
        };
        let derived = crate::derive_incremental(&session, "", &[], 2_000)
            .await
            .unwrap();
        let reference = rendered_text(&derived.messages[..1]);
        assert!(reference.contains("[user message 1500]"));
        assert!(reference.contains("[user message 1513]"));
        assert!(reference.contains("Do not push"));
        assert!(
            codetether_request_tokens::estimate_request_tokens("", &derived.messages, &[]) <= 2_000
        );
    }
}
