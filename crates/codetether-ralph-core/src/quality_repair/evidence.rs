//! Update failure diagnostics and append repair output.
use super::QualityProgress;
impl QualityProgress {
    /// Keep the existing failure when a new diagnostic is empty.
    ///
    /// # Examples
    /// ```
    /// # use codetether_ralph_core::quality_repair::QualityProgress;
    /// # let mut state = QualityProgress { output: String::new(), qg_passed: false, failure_reason: None };
    /// state.remember_failure("failed");
    /// assert_eq!(state.failure_reason.as_deref(), Some("failed"));
    /// ```
    pub fn remember_failure(&mut self, error_output: &str) {
        if !error_output.is_empty() {
            self.failure_reason = Some(error_output.to_string());
        }
    }
    /// Append one repair response with its one-based attempt number.
    ///
    /// # Examples
    /// ```
    /// # use codetether_ralph_core::quality_repair::QualityProgress;
    /// # let mut state = QualityProgress { output: String::new(), qg_passed: false, failure_reason: None };
    /// state.append_repair("fixed".into(), 0);
    /// assert!(state.output.contains("[repair attempt 1] fixed"));
    /// ```
    pub fn append_repair(&mut self, repair_output: String, qg_attempt: usize) {
        let output = &mut self.output;
        *output = format!(
            "{output}\n---\n[repair attempt {}] {repair_output}",
            qg_attempt + 1
        );
    }
}
