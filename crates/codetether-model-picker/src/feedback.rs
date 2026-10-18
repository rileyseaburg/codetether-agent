//! Status feedback for picker opening and background refresh completion.
use crate::ModelPickerState;

impl ModelPickerState {
    /// Open the picker and replace the frontend's status with its initial message.
    /// # Examples
    /// ```
    /// let mut picker = codetether_model_picker::ModelPickerState::default();
    /// let mut status = String::new();
    /// picker.open(&mut status);
    /// assert_eq!(status, "Model picker");
    /// ```
    pub fn open(&mut self, status: &mut String) {
        *status = self.open_with_status();
    }

    /// Drain refresh results and update status only when feedback is available.
    /// # Arguments
    /// * `view_active` — Whether the frontend currently displays the model view.
    /// * `status` — Frontend status line to update when feedback is available.
    /// # Examples
    /// ```
    /// let mut picker = codetether_model_picker::ModelPickerState::default();
    /// let mut status = "Ready".to_string();
    /// picker.apply_refresh_status(false, &mut status);
    /// assert_eq!(status, "Ready");
    /// ```
    pub fn apply_refresh_status(&mut self, view_active: bool, status: &mut String) {
        if let Some(next) = self.drain_refresh_status(view_active) {
            *status = next;
        }
    }

    /// Open the picker and return its initial status message.
    ///
    /// # Returns
    /// The status to display after resetting the picker filter and selection.
    /// # Examples
    /// ```
    /// let mut picker = codetether_model_picker::ModelPickerState::default();
    /// assert_eq!(picker.open_with_status(), "Model picker");
    /// assert!(picker.model_picker_active);
    /// ```
    pub fn open_with_status(&mut self) -> String {
        self.open_model_picker();
        "Model picker".to_string()
    }

    /// Drain background refresh results and select feedback for the active UI.
    ///
    /// # Arguments
    /// * `view_active` — Whether the application's current view is the model view.
    /// # Returns
    /// A completion message when the picker is open or its view is active;
    /// `None` leaves the application's status unchanged. Refresh results are
    /// still drained and applied when the picker is hidden.
    /// # Examples
    /// ```
    /// let mut picker = codetether_model_picker::ModelPickerState::default();
    /// assert!(picker.drain_refresh_status(false).is_none());
    /// ```
    pub fn drain_refresh_status(&mut self, view_active: bool) -> Option<String> {
        let summary = self.drain_model_refresh()?;
        if self.model_picker_active || view_active {
            Some(summary.status_message())
        } else {
            None
        }
    }
}
