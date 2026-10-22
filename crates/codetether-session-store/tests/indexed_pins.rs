mod constraint_support;
use codetether_context_policy::pages::PageKind;
use codetether_session_store::{load_sync, pin_edits, save, window::WINDOW};
use constraint_support::{Session, message};

async fn seeded(path: &std::path::Path) -> Session {
    let mut session = Session::default();
    for text in ["old requirement", "new correction"] {
        session.messages.push(message(text));
        session.pages.push(PageKind::Constraint);
    }
    for _ in 0..WINDOW + 20 {
        session.messages.push(message("later"));
        session.pages.push(PageKind::Conversation);
    }
    save(&session, path).await.unwrap();
    load_sync::<Session>(path, WINDOW, |_| Ok(()))
        .unwrap()
        .session
}

#[tokio::test]
async fn sparse_edits_preserve_history_and_emit_only_selected_put_events() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("pins.json");
    let tail = seeded(&path).await;
    let before = tail.messages.len();
    pin_edits::prepare(&tail, &[0, 1]).await.unwrap();
    {
        let mut state = tail.state.0.lock().unwrap();
        assert_eq!(state.pin_sources.len(), 2);
        state.pin_sources[0].page = PageKind::SupersededConstraint { by_turn: 1 };
        state.pin_sources[1].page = PageKind::Conversation;
    }
    save(&tail, &path).await.unwrap();
    assert_eq!(tail.messages.len(), before);
    assert!(tail.state.0.lock().unwrap().pin_sources.is_empty());
    let full = load_sync::<Session>(&path, usize::MAX, |_| Ok(()))
        .unwrap()
        .session;
    assert_eq!(full.pages[0], PageKind::SupersededConstraint { by_turn: 1 });
    assert_eq!(full.pages[1], PageKind::Conversation);
    assert_eq!(full.messages.len(), WINDOW + 22);
    let db = codetether_session_store::connection::open(&path).unwrap();
    let puts: usize = db
        .query_row(
            "SELECT COUNT(*) FROM events WHERE revision=2 AND op='put'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(
        puts, 2,
        "no intervening record is copied into the audit log"
    );
    assert_eq!(
        codetether_session_store::constraints::prefix(&tail).len(),
        1
    );
}

#[tokio::test]
async fn changed_second_source_rolls_back_both_edits_and_revision() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("pins.json");
    let tail = seeded(&path).await;
    pin_edits::prepare(&tail, &[0, 1]).await.unwrap();
    {
        let mut state = tail.state.0.lock().unwrap();
        state.pin_sources[0].page = PageKind::Conversation;
        state.pin_sources[1].original = PageKind::Evidence;
        state.pin_sources[1].page = PageKind::Conversation;
    }
    assert!(
        save(&tail, &path)
            .await
            .unwrap_err()
            .to_string()
            .contains("SESSION_REVISION_CONFLICT")
    );
    let full = load_sync::<Session>(&path, usize::MAX, |_| Ok(()))
        .unwrap()
        .session;
    assert_eq!(full.pages[0], PageKind::Constraint);
    assert_eq!(full.pages[1], PageKind::Constraint);
    assert_eq!(full.state.0.lock().unwrap().revision, 1);
}

#[tokio::test]
async fn concurrent_revision_invalidates_staged_edits_and_reads() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("pins.json");
    let tail = seeded(&path).await;
    pin_edits::prepare(&tail, &[0]).await.unwrap();
    tail.state.0.lock().unwrap().pin_sources[0].page = PageKind::Conversation;
    let mut other = load_sync::<Session>(&path, WINDOW, |_| Ok(()))
        .unwrap()
        .session;
    other.append("concurrent write");
    save(&other, &path).await.unwrap();
    assert!(
        save(&tail, &path)
            .await
            .unwrap_err()
            .to_string()
            .contains("SESSION_REVISION_CONFLICT")
    );
    let stale = load_sync::<Session>(&path, WINDOW, |_| Ok(()))
        .unwrap()
        .session;
    other.append("another write");
    save(&other, &path).await.unwrap();
    assert!(
        pin_edits::prepare(&stale, &[0])
            .await
            .unwrap_err()
            .to_string()
            .contains("SESSION_REVISION_CONFLICT")
    );
}

#[tokio::test]
async fn invalid_or_missing_sources_do_not_prepare_partial_edits() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("pins.json");
    let tail = seeded(&path).await;
    assert!(pin_edits::prepare(&tail, &[0, 1, 2]).await.is_err());
    assert!(pin_edits::prepare(&tail, &[0, usize::MAX]).await.is_err());
    assert!(tail.state.0.lock().unwrap().pin_sources.is_empty());
    let db = codetether_session_store::connection::open(&path).unwrap();
    db.execute(
        "DELETE FROM records WHERE session_id='constraints-session' AND kind=0 AND seq=1",
        [],
    )
    .unwrap();
    assert!(pin_edits::prepare(&tail, &[0, 1]).await.is_err());
    assert!(tail.state.0.lock().unwrap().pin_sources.is_empty());
}
