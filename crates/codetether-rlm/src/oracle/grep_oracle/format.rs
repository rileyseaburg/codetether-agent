//! Interpret claimed answer representations.

use super::{GrepOracle, GrepVerification};
use crate::oracle::FinalAnswerFormat;
impl GrepOracle {
    pub(super) fn verify_format(
        &self,
        format: FinalAnswerFormat,
        ground_truth: &[(usize, String)],
    ) -> GrepVerification {
        // Compare based on answer format
        match format {
            FinalAnswerFormat::LineNumberedMatches { matches: claimed } => {
                self.verify_matches(&claimed, &ground_truth)
            }
            FinalAnswerFormat::CountResult {
                count: claimed_count,
            } => {
                let actual_count = ground_truth.len();
                if claimed_count == actual_count {
                    GrepVerification::ExactMatch
                } else {
                    GrepVerification::SubsetMatch {
                        claimed: claimed_count,
                        actual: actual_count,
                    }
                }
            }
            FinalAnswerFormat::StructuredData { .. } => GrepVerification::CannotVerify {
                reason: "Structured data not supported by grep oracle".to_string(),
            },
            FinalAnswerFormat::FreeFormText { text } => {
                // Try to extract line numbers from free-form text
                if let Some(extracted) = self.extract_line_numbers_from_text(&text) {
                    self.verify_matches(&extracted, &ground_truth)
                } else {
                    GrepVerification::CannotVerify {
                        reason: "Could not extract line numbers from free-form text".to_string(),
                    }
                }
            }
        }
    }
}
