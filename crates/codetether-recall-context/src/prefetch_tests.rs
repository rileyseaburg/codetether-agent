//! Automatic recall never crosses conversations sharing a directory.
#[path = "prefetch_fixture.rs"]
mod fixture;
use fixture::Fixture;

#[tokio::test]
async fn mux_sessions_share_workspace_not_automatic_history() {
    let mut host = Fixture::new();
    let mut catalog =
        codetether_recall_store::RecallCatalog::new(host.dir.path().canonicalize().unwrap());
    for id in ["quote-session", "ads-session"] {
        catalog.insert(id);
    }
    codetether_recall_store::write_catalog(&catalog, &host)
        .await
        .unwrap();
    for (own, other) in [
        ("quote-session", "ads-session"),
        ("ads-session", "quote-session"),
    ] {
        host.reads.lock().unwrap().clear();
        let session = host.session(own);
        let message = crate::message(&session, &host).await.unwrap();
        let text = codetether_context_policy::transcript::extract_text_content(&message.content);
        assert!(text.contains("Earlier evidence from this session"));
        assert!(text.contains(own));
        assert!(!text.contains(other));
        assert_eq!(*host.reads.lock().unwrap(), [own]);
    }

    // Explicit cross-session queries still work; only automatic injection is scoped.
    let hits = codetether_recall_query::run(
        host.dir.path(),
        "quote template include",
        codetether_recall_query::SearchOptions {
            session_id: None,
            excluded_session: None,
            limit: 3,
            minimum_score: 0.45,
        },
        &host,
    )
    .await
    .unwrap();
    assert_eq!(hits.len(), 2);

    // A new session without indexed history must not borrow a sibling's notes.
    host.projections.remove("quote-session");
    host.reads.lock().unwrap().clear();
    let mut session = host.session("quote-session");
    assert!(crate::message(&session, &host).await.is_none());
    assert_eq!(*host.reads.lock().unwrap(), ["quote-session"]);

    // The existing history-access prohibition remains authoritative.
    session.allowed = false;
    host.reads.lock().unwrap().clear();
    assert!(crate::message(&session, &host).await.is_none());
    assert!(host.reads.lock().unwrap().is_empty());
    session.allowed = true;
    session.messages.clear();
    assert!(crate::message(&session, &host).await.is_none());
}
