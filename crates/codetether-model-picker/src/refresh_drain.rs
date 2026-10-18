//! Accept the newest completion and restore the target-model selection.
use crate::ModelPickerState;
use codetether_model_discovery::{ModelRefreshEvent, ModelRefreshSummary};
impl ModelPickerState {
    /// Applies the newest available completion and returns its summary.
    ///
    /// Returns `None` when the receiver has no pending completion. Accepted
    /// results are persisted before restoring the target-model selection.
    /// # Examples
    /// ```
    /// let mut state = codetether_model_picker::ModelPickerState::default();
    /// assert!(state.drain_model_refresh().is_none());
    /// ```
    pub fn drain_model_refresh(&mut self) -> Option<ModelRefreshSummary> {
        let mut latest = None;
        if let Some(rx) = self.model_refresh_rx.as_mut() {
            while let Ok(event) = rx.try_recv() {
                latest = Some(event);
            }
        }

        let Some(ModelRefreshEvent::Loaded { models, summary }) = latest else {
            return None;
        };

        self.finish_model_refresh(models);

        if let Some(target) = self.model_picker_target_model.as_deref()
            && let Some(index) = self
                .filtered_models()
                .iter()
                .position(|model| *model == target)
        {
            self.selected_model_index = index;
        }

        Some(summary)
    }
}
