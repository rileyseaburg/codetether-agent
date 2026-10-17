//! Okr validate behavior.

use crate::Okr;
use crate::OkrValidationError;
impl Okr {
    /// Validate the OKR structure
    ///
    /// # Errors
    /// Returns the first validation error for missing required fields or invalid values.
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{Okr};
    /// let mut value = Okr::new("Reliability", "Reduce failures");
    /// assert!(value.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), OkrValidationError> {
        if self.title.trim().is_empty() {
            return Err(OkrValidationError::EmptyTitle);
        }
        if self.key_results.is_empty() {
            return Err(OkrValidationError::NoKeyResults);
        }
        for kr in &self.key_results {
            kr.validate()?;
        }
        Ok(())
    }
}
