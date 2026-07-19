use super::{provider, request};
use crate::provider::{ContentPart, StreamChunk};
use crate::session::helper::stream::restart::{RestartPolicy, run};

#[tokio::test(start_paused = true)]
async fn completed_text_is_sent_on_retry_and_committed_once() {
    let first = ContentPart::Text {
        text: "first".into(),
    };
    let (concrete, provider) = provider(vec![
        vec![
            StreamChunk::Text("first".into()),
            StreamChunk::OutputItemDone {
                content: first.clone(),
            },
            StreamChunk::Error("codex-retryable: reset".into()),
        ],
        vec![
            StreamChunk::Text("second".into()),
            StreamChunk::Done { usage: None },
        ],
    ]);
    let response = run(
        &provider,
        request(),
        "session",
        None,
        &RestartPolicy::default(),
    )
    .await
    .unwrap();
    assert!(
        matches!(&response.message.content[..], [ContentPart::Text { text: a }, ContentPart::Text { text: b }] if a == "first" && b == "second")
    );
    assert!(
        matches!(&concrete.requests.lock().unwrap()[1].messages[0].content[0], ContentPart::Text { text } if text == "first")
    );
}
