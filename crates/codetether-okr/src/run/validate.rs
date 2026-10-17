//! OkrRun validate behavior.

use crate::OkrRun;
use crate::OkrValidationError;
impl OkrRun {
    /// Validate the run
    ///
    /// # Errors
    /// Returns the first validation error for missing required fields or invalid values.
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{OkrRun};
    /// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
    /// assert!(value.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), OkrValidationError> {
        if self.name.trim().is_empty() {
            return Err(OkrValidationError::EmptyRunName);
        }
        Ok(())
    }
}
