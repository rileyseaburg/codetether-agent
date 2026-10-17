//! Parse FINAL JSON while retaining malformed source.

use super::FinalPayload;
impl FinalPayload {
    /// Parse a JSON string into a FinalPayload.
    ///
    /// Returns `FinalPayload::Malformed` if parsing fails.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{FinalPayload};
    /// assert!(matches!(FinalPayload::parse("not JSON"), FinalPayload::Malformed { .. }));
    /// ```
    pub fn parse(json_str: &str) -> Self {
        let trimmed = json_str.trim();

        // Try to parse as JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(trimmed);

        match parsed {
            Ok(value) => Self::from_value(value, trimmed),
            Err(e) => {
                // Not valid JSON at all
                FinalPayload::Malformed {
                    raw: trimmed.to_string(),
                    error: format!("JSON parse error: {}", e),
                }
            }
        }
    }
}
