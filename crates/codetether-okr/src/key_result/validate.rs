//! KeyResult validate behavior.

use crate::KeyResult;
use crate::OkrValidationError;
impl KeyResult {
    /// Validate the key result
    ///
    /// # Errors
    /// Returns the first validation error for missing required fields or invalid values.
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KeyResult};
    /// let mut value = KeyResult::new(uuid::Uuid::nil(), "Checks", 10.0, "count");
    /// assert!(value.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), OkrValidationError> {
        if self.title.trim().is_empty() {
            return Err(OkrValidationError::EmptyKeyResultTitle);
        }
        if self.target_value < 0.0 {
            return Err(OkrValidationError::InvalidTargetValue);
        }
        Ok(())
    }
}
