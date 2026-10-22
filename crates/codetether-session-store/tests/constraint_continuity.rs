mod constraint_support;
use codetether_context_policy::{pages::PageKind, state_header::prepend_state_header};
use codetether_provider_core::ContentPart;
use codetether_session_store::{constraints, load_sync, save, window::WINDOW};
use constraint_support::{Session, message};

#[tokio::test]
async fn full_sources_survive_cold_restart_eviction_and_explicit_supersession() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("constraints-session.json");
    let mut session = Session::default();
    for index in 0..21 {
        session.messages.push(message(&format!(
            "constraint {index}\n{}\nEND {index}",
            "🦀".repeat(200)
        )));
        session.pages.push(PageKind::Constraint);
    }
    for _ in 0..WINDOW {
        session.messages.push(message("later"));
        session.pages.push(PageKind::Conversation);
    }
    save(&session, &path).await.unwrap();
    let mut tail = load_sync::<Session>(&path, WINDOW, |_| Ok(()))
        .unwrap()
        .session;
    assert_eq!(constraints::prefix(&tail).len(), 21);
    assert!(constraints::prefix(&tail)[0].text.ends_with("END 0"));
    tail.messages[3] = message("an additional durable correction\nkeep this complete ending");
    tail.pages[3] = PageKind::Constraint;
    save(&tail, &path).await.unwrap();
    for _ in 0..5 {
        tail.append("continue");
        save(&tail, &path).await.unwrap();
    }
    let reloaded = load_sync::<Session>(&path, WINDOW, |_| Ok(()))
        .unwrap()
        .session;
    let pins = constraints::prefix(&reloaded);
    assert_eq!(pins.len(), 22);
    assert!(
        pins.iter()
            .any(|p| p.source_turn == 24 && p.text.ends_with("keep this complete ending"))
    );
    let mut full = load_sync::<Session>(&path, usize::MAX, |_| Ok(()))
        .unwrap()
        .session;
    full.pages[0] = PageKind::SupersededConstraint { by_turn: 1 };
    save(&full, &path).await.unwrap();
    let final_tail = load_sync::<Session>(&path, WINDOW, |_| Ok(()))
        .unwrap()
        .session;
    let final_pins = constraints::prefix(&final_tail);
    assert_eq!(final_pins[0].superseded_by, Some(1));
    let mut messages = vec![];
    prepend_state_header(
        final_pins,
        &final_tail.pages,
        &final_tail.messages,
        || final_tail.state.message_offset(),
        &mut messages,
    );
    let ContentPart::Text { text } = &messages[0].content[0] else {
        unreachable!()
    };
    assert!(!text.contains("constraint 0\n"));
    assert!(text.contains("constraint 1\n"));
    assert!(text.contains("supersedes [0]"));
}
