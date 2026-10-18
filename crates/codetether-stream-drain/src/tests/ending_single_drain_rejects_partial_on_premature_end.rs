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
async fn single_drain_rejects_partial_on_premature_end() {
    let s = Box::pin(stream::iter(vec![StreamChunk::Text("partial".to_string())]));
    let error = collect_stream_completion_with_events(s, None)
        .await
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "temporary provider availability issue; retry the request"
    );
    assert!(!error.to_string().to_ascii_lowercase().contains("websocket"));
}
