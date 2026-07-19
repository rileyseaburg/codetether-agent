use super::{provider, request};
use crate::provider::{ContentPart, StreamChunk};
use crate::session::Session;
use crate::session::helper::stream::restart::{RestartPolicy, run};

#[tokio::test(start_paused = true)]
async fn exhausted_checkpoint_is_saved_to_session() {
    let content = ContentPart::Text {
        text: "durable".into(),
    };
    let (_, provider) = provider(vec![vec![
        StreamChunk::OutputItemDone { content },
        StreamChunk::Error("codex-retryable: reset".into()),
    ]]);
    let policy = RestartPolicy {
        max_restarts: 0,
        ..RestartPolicy::default()
    };
    let error = run(&provider, request(), "session", None, &policy)
        .await
        .unwrap_err();
    let mut session = Session::new().await.unwrap();
    super::super::super::prompt_call::persist_stream_checkpoints(&mut session, &error)
        .await
        .unwrap();
    let loaded = Session::load(&session.id).await.unwrap();
    Session::delete(&session.id).await.unwrap();
    assert!(
        matches!(&loaded.messages[0].content[..], [ContentPart::Text { text }] if text == "durable")
    );
}
