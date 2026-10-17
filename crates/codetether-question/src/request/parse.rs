//! Decode modern or legacy JSON, then construct a validated request.
use super::{Question, Request};
use crate::Error;
impl Request {
    /// Parse modern `questions` batches or legacy `question/options/default` arguments.
    ///
    /// # Arguments
    /// * `value` — JSON tool arguments; runtime-injected metadata is ignored.
    /// # Returns
    /// A validated request. Legacy defaults are presented first, not auto-answered.
    /// # Errors
    /// Returns [`Error::Invalid`] for malformed, empty, or duplicate choice labels.
    /// # Examples
    /// ```
    /// assert!(codetether_question::Request::parse(serde_json::json!({"questions":[]})).is_err());
    /// ```
    pub fn parse(value: serde_json::Value) -> Result<Self, Error> {
        let questions = if let Some(batch) = value.get("questions") {
            serde_json::from_value::<Vec<Question>>(batch.clone())
                .map_err(|e| Error::Invalid(e.to_string()))?
        } else {
            super::legacy::decode(value)?
        };
        super::validate::validate(&questions)?;
        Ok(Self { questions })
    }
}
