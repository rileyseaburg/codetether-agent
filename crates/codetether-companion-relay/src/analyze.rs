//! Inference abstraction so transport and provider wiring stay separate.
use futures_util::future::BoxFuture;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// Streamed text callback; the relay ignores calls after cancellation.
pub type Delta = Box<dyn FnMut(&str) + Send>;

/// One bounded vision request. `image` is memory-only base64 JPEG.
pub struct Analysis {
    /// Base64 JPEG; never log it.
    pub image: String,
    /// Explicit `provider/model` chosen by the owner.
    pub model: String,
    /// Owner prompt or fresh-frame question.
    pub prompt: String,
    /// Previous analysis, supplied as untrusted context.
    pub previous: String,
    /// Cancelled on pause, stop, timeout, or budget overflow.
    pub cancel: CancellationToken,
    /// Receives streamed analysis text.
    pub delta: Delta,
}

/// Shared analyzer: resolves after the provider sends `[DONE]`.
///
/// ```
/// use codetether_companion_relay::{Analysis, Analyze};
/// use std::sync::Arc;
/// let analyze: Analyze = Arc::new(|mut input: Analysis| {
///     Box::pin(async move { (input.delta)("ok"); Ok(()) })
/// });
/// # let _ = analyze;
/// ```
pub type Analyze = Arc<dyn Fn(Analysis) -> BoxFuture<'static, anyhow::Result<()>> + Send + Sync>;
