use super::{provider, request};
use crate::provider::{ContentPart, StreamChunk};
use crate::session::helper::stream::checkpointed_content;
use crate::session::helper::stream::restart::{RestartPolicy, run};

#[tokio::test(start_paused = true)]
async fn exhaustion_carries_completed_content() {
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
    assert!(
        matches!(&checkpointed_content(&error).unwrap()[..], [ContentPart::Text { text }] if text == "durable")
    );
}
