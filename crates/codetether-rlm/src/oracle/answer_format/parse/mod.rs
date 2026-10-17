//! Answer format priority: numbered matches, counts, JSON, then free text.

mod count;
mod numbered;
use super::FinalAnswerFormat;
use count::extract_count_from_text;
impl FinalAnswerFormat {
    /// Parse a FINAL() answer string into its classified format.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::oracle::FinalAnswerFormat;
    /// assert!(matches!(FinalAnswerFormat::parse("Found 2 matches"),FinalAnswerFormat::CountResult{count:2}));
    /// ```
    pub fn parse(answer: &str) -> Self {
        if let Some(format) = numbered::parse(answer) {
            return format;
        }
        // Try to parse as count result
        let lower = answer.to_lowercase();
        if lower.contains("found") || lower.contains("count:") || lower.contains("occurrences") {
            // Extract number from text like "Found 15 async functions"
            if let Some(count) = extract_count_from_text(answer) {
                return Self::CountResult { count };
            }
        }

        // Try to parse as JSON
        if (answer.trim().starts_with('{') || answer.trim().starts_with('['))
            && let Ok(data) = serde_json::from_str::<serde_json::Value>(answer)
        {
            return Self::StructuredData { data };
        }

        // Default to free-form text
        Self::FreeFormText {
            text: answer.to_string(),
        }
    }
}
#[cfg(test)]
mod tests;
