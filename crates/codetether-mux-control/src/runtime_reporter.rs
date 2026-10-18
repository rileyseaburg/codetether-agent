//! Change-driven semantic status reporting and orderly clearing.
use codetether_mux_model::{MuxRuntimeStatus, RuntimeView};
/// Last status attempted for one inherited mux session.
/// # Examples
/// ```
/// let _reporter = codetether_mux_control::RuntimeReporter::default();
/// ```
#[derive(Default)]
pub struct RuntimeReporter {
    last: Option<MuxRuntimeStatus>,
}
impl RuntimeReporter {
    /// Report only changed facts, caching before the best-effort send.
    pub async fn update(&mut self, view: RuntimeView<'_>) {
        if self.last.as_ref().is_some_and(|last| view.unchanged(last)) {
            return;
        }
        let status = view.snapshot();
        self.last = Some(status.clone());
        if let Err(error) = crate::report_runtime(Some(status)).await {
            tracing::debug!(%error,"Mux runtime status unavailable");
        }
    }
    /// Clear the inherited report if this reporter has attempted any update.
    pub async fn clear(self) {
        if self.last.is_some()
            && let Err(error) = crate::report_runtime(None).await
        {
            tracing::debug!(%error,"Could not clear mux runtime status");
        }
    }
}
