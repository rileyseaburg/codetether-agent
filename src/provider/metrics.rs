//! Provider metrics wrapper
//!
//! Wraps any `Provider` to automatically record latency, throughput,
//! and tokens-per-second via the global `PROVIDER_METRICS` registry.

use super::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk, Usage};
use crate::telemetry::{PROVIDER_METRICS, ProviderRequestRecord};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// A provider wrapper that instruments every call with performance metrics.
pub struct MetricsProvider {
    inner: Arc<dyn Provider>,
}

impl MetricsProvider {
    /// Wrap a provider with automatic metrics collection
    pub fn wrap(inner: Arc<dyn Provider>) -> Arc<Self> {
        Arc::new(Self { inner })
    }

    async fn record_request(&self, model: &str, latency_ms: u64, usage: &Usage, success: bool) {
        let record = ProviderRequestRecord {
            provider: self.inner.name().to_string(),
            model: model.to_string(),
            timestamp: chrono::Utc::now(),
            prompt_tokens: usage.prompt_tokens as u64,
            completion_tokens: usage.completion_tokens as u64,
            input_tokens: usage.prompt_tokens as u64,
            output_tokens: usage.completion_tokens as u64,
            latency_ms,
            ttft_ms: None, // non-streaming: no TTFT distinction
            success,
        };

        tracing::info!(
            provider = %record.provider,
            model = %record.model,
            latency_ms = record.latency_ms,
            input_tokens = record.input_tokens,
            output_tokens = record.output_tokens,
            tps = format!("{:.1}", record.tokens_per_second()),
            "Provider request completed"
        );

        PROVIDER_METRICS.record(record).await;
    }
}

#[async_trait]
impl Provider for MetricsProvider {
    fn name(&self) -> &str {
        self.inner.name()
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.inner.list_models().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let model = request.model.clone();
        let start = std::time::Instant::now();

        match self.inner.complete(request).await {
            Ok(response) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                self.record_request(&model, latency_ms, &response.usage, true)
                    .await;
                Ok(response)
            }
            Err(e) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                self.record_request(&model, latency_ms, &Usage::default(), false)
                    .await;
                Err(e)
            }
        }
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let model = request.model.clone();
        let provider_name = self.inner.name().to_string();
        let start = std::time::Instant::now();

        match self.inner.complete_stream(request).await {
            Ok(stream) => {
                let ttft_ms = start.elapsed().as_millis() as u64;

                // Wrap the stream to capture final usage from Done chunk
                let stream =
                    StreamMetricsWrapper::new(stream, provider_name, model, start, ttft_ms);

                Ok(Box::pin(stream))
            }
            Err(e) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                let record = ProviderRequestRecord {
                    provider: provider_name,
                    model,
                    timestamp: chrono::Utc::now(),
                    prompt_tokens: 0,
                    completion_tokens: 0,
                    input_tokens: 0,
                    output_tokens: 0,
                    latency_ms,
                    ttft_ms: None,
                    success: false,
                };
                PROVIDER_METRICS.record(record).await;
                Err(e)
            }
        }
    }
}

/// Wraps a stream to capture metrics when the `Done` chunk arrives
struct StreamMetricsWrapper {
    inner: futures::stream::BoxStream<'static, StreamChunk>,
    provider: String,
    model: String,
    start: std::time::Instant,
    ttft_ms: u64,
    recorded: bool,
}

impl StreamMetricsWrapper {
    fn new(
        inner: futures::stream::BoxStream<'static, StreamChunk>,
        provider: String,
        model: String,
        start: std::time::Instant,
        ttft_ms: u64,
    ) -> Self {
        Self {
            inner,
            provider,
            model,
            start,
            ttft_ms,
            recorded: false,
        }
    }
}

impl futures::Stream for StreamMetricsWrapper {
    type Item = StreamChunk;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        use std::task::Poll;

        let result = std::pin::Pin::new(&mut self.inner).poll_next(cx);

        match &result {
            Poll::Ready(Some(StreamChunk::Done { usage })) if !self.recorded => {
                self.recorded = true;
                let latency_ms = self.start.elapsed().as_millis() as u64;
                let (input_tokens, output_tokens) = usage
                    .as_ref()
                    .map(|u| (u.prompt_tokens as u64, u.completion_tokens as u64))
                    .unwrap_or((0, 0));

                let record = ProviderRequestRecord {
                    provider: self.provider.clone(),
                    model: self.model.clone(),
                    timestamp: chrono::Utc::now(),
                    prompt_tokens: input_tokens,
                    completion_tokens: output_tokens,
                    input_tokens,
                    output_tokens,
                    latency_ms,
                    ttft_ms: Some(self.ttft_ms),
                    success: true,
                };

                tracing::info!(
                    provider = %record.provider,
                    model = %record.model,
                    latency_ms = record.latency_ms,
                    ttft_ms = record.ttft_ms,
                    input_tokens = record.input_tokens,
                    output_tokens = record.output_tokens,
                    tps = format!("{:.1}", record.tokens_per_second()),
                    "Provider streaming request completed"
                );

                let metrics = PROVIDER_METRICS.clone();
                tokio::spawn(async move { metrics.record(record).await });
            }
            Poll::Ready(Some(StreamChunk::Error(_))) if !self.recorded => {
                self.recorded = true;
                let latency_ms = self.start.elapsed().as_millis() as u64;
                let record = ProviderRequestRecord {
                    provider: self.provider.clone(),
                    model: self.model.clone(),
                    timestamp: chrono::Utc::now(),
                    prompt_tokens: 0,
                    completion_tokens: 0,
                    input_tokens: 0,
                    output_tokens: 0,
                    latency_ms,
                    ttft_ms: Some(self.ttft_ms),
                    success: false,
                };
                let metrics = PROVIDER_METRICS.clone();
                tokio::spawn(async move { metrics.record(record).await });
            }
            _ => {}
        }

        result
    }
}
