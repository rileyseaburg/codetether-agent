//! Stream wrapper state for first-output latency measurement.

use super::super::{StreamChunk, Usage};

#[path = "stream/capture.rs"]
mod capture;
#[cfg(test)]
#[path = "stream/tests.rs"]
mod tests;

/// State retained while observing a provider stream.
pub(super) struct MetricsStream {
    pub(super) inner: futures::stream::BoxStream<'static, StreamChunk>,
    pub(super) provider: String,
    pub(super) model: String,
    pub(super) start: std::time::Instant,
    pub(super) ttft_ms: Option<u64>,
    pub(super) recorded: bool,
}

impl MetricsStream {
    /// Creates an unobserved stream whose TTFT starts at request dispatch.
    pub(super) fn new(
        inner: futures::stream::BoxStream<'static, StreamChunk>,
        provider: String,
        model: String,
        start: std::time::Instant,
    ) -> Self {
        Self {
            inner,
            provider,
            model,
            start,
            ttft_ms: None,
            recorded: false,
        }
    }

    pub(super) fn record(&mut self, usage: Option<Usage>, success: bool) {
        self.recorded = true;
        let (provider, model, start, ttft) = (
            self.provider.clone(),
            self.model.clone(),
            self.start,
            self.ttft_ms,
        );
        tokio::spawn(async move {
            super::record::stream(&provider, &model, start, ttft, usage.as_ref(), success).await;
        });
    }
}
