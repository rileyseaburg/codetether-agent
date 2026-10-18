//! Original single-drain regression case.
//! Tests for how the single-drain collector treats stream termination.
//!
//! Distinct from the restart engine (see `srp_tests`): this leaf collector has
//! no restart budget, so a premature end is rejected rather than committing a
//! response that may be truncated.

use super::collect_stream_completion_with_events;

use codetether_provider_core::{ContentPart, StreamChunk};

use futures::{StreamExt, stream};

#[tokio::test]
async fn done_finishes_without_waiting_for_transport_close() {
    let terminal = stream::iter(vec![
        StreamChunk::Text("complete".to_string()),
        StreamChunk::Done { usage: None },
    ]);
    let trailing = stream::once(async { panic!("collector polled past terminal Done") });
    let open = terminal.chain(trailing);
    let response = collect_stream_completion_with_events(Box::pin(open), None)
        .await
        .unwrap();
    assert!(matches!(
        &response.message.content[0],
        ContentPart::Text { text } if text == "complete"
    ));
}
