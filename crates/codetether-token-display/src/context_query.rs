//! Obtain the current-turn context usage indicator.
use codetether_telemetry::TokenUsageSnapshot;
use ratatui::style::Style;
impl crate::TokenDisplay {
    /// Always-on context-window indicator `(label, style)` for the active
    /// model, based on the current turn's prompt size. Delegates to
    /// [`crate::context_indicator`].
    pub(crate) fn get_context_indicator(
        &self,
        model_snapshots: &[TokenUsageSnapshot],
    ) -> Option<(String, Style)> {
        crate::context::context_indicator(self, model_snapshots)
    }
}
