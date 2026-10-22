mod constraint_support;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_session_store::{load_sync, pin_edits, save, window::WINDOW};
use constraint_support::{Session, message};

#[tokio::test]
async fn indexed_page_changes_preserve_blob_text_and_recorded_tool_outcomes() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("evidence.json");
    let mut session = Session::default();
    session.messages.push(Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "read".into(),
            name: "read".into(),
            arguments: "{}".into(),
            thought_signature: None,
        }],
    });
    session.messages.push(Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "read".into(),
            content: "observed output".into(),
        }],
    });
    let original = format!("{}\nKEEP COMPLETE END", "🦀".repeat(20_000));
    session.messages.push(message(&original));
    session.pages = vec![
        PageKind::Conversation,
        PageKind::Evidence,
        PageKind::Constraint,
    ]
    .into();
    for _ in 0..WINDOW {
        session.append("later");
    }
    save(&session, &path).await.unwrap();
    let db = codetether_session_store::connection::open(&path).unwrap();
    db.execute(
        "INSERT INTO consumers VALUES ('constraints-session','rlm',16,0)",
        [],
    )
    .unwrap();
    db.execute(
        "INSERT INTO projections VALUES ('constraints-session','rlm',0,16,'{}')",
        [],
    )
    .unwrap();
    let tail = load_sync::<Session>(&path, WINDOW, |_| Ok(()))
        .unwrap()
        .session;
    pin_edits::prepare(&tail, &[2]).await.unwrap();
    assert_eq!(
        tail.state.0.lock().unwrap().pin_sources[0]
            .message
            .content
            .len(),
        1
    );
    tail.state.0.lock().unwrap().pin_sources[0].page = PageKind::Conversation;
    save(&tail, &path).await.unwrap();
    let full = load_sync::<Session>(&path, usize::MAX, |_| Ok(()))
        .unwrap()
        .session;
    let ContentPart::Text { text } = &full.messages[2].content[0] else {
        panic!()
    };
    assert_eq!(text, &original);
    let state: (String, usize) = db
        .query_row(
            "SELECT state,result_seq FROM tool_calls WHERE call_id='read'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(state, ("result_recorded".into(), 1));
    let generation: (usize, usize) = db
        .query_row(
            "SELECT seq,revision FROM consumers WHERE name='rlm'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(generation, (0, 1));
    let projections: usize = db
        .query_row("SELECT COUNT(*) FROM projections", [], |row| row.get(0))
        .unwrap();
    assert_eq!(projections, 0);
}
