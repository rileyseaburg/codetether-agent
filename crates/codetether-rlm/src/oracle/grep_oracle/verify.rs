//! Infer a query and evaluate its answer against source matches.

use super::{GrepOracle, GrepVerification};
use crate::oracle::FinalAnswerFormat;
impl GrepOracle {
    /// Verify a FINAL() answer against ground truth.
    ///
    /// Takes the claimed answer and the original query, infers the pattern,
    /// runs grep, and compares results.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GrepOracle, GrepVerification};
    /// let oracle = GrepOracle::new("async fn main() {}".into());
    /// assert_eq!(oracle.verify("Found 1 async functions", "count async functions"), GrepVerification::ExactMatch);
    /// ```
    pub fn verify(&self, answer: &str, query: &str) -> GrepVerification {
        let format = FinalAnswerFormat::parse(answer);

        // Infer the grep pattern from the query
        let pattern = Self::infer_pattern(query)
            .or_else(|| Self::infer_pattern(answer))
            .ok_or_else(|| "Could not infer grep pattern from query".to_string());
        let pattern = match pattern {
            Ok(p) => p,
            Err(reason) => {
                return GrepVerification::CannotVerify { reason };
            }
        };

        // Get ground truth
        let ground_truth = match self.grep(&pattern) {
            Ok(m) => m,
            Err(e) => {
                return GrepVerification::CannotVerify {
                    reason: format!("Grep failed: {}", e),
                };
            }
        };

        self.verify_format(format, &ground_truth)
    }
}
