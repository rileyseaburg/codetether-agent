//! Compare ordered matches and set differences.

use super::{GrepOracle, GrepVerification};
use std::collections::HashSet;
impl GrepOracle {
    /// Verify claimed matches against ground truth (public for validator use).
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GrepOracle, GrepVerification};
    /// let oracle = GrepOracle::new("a".into());
    /// let matches = vec![(1, "a".into())];
    /// assert_eq!(oracle.verify_matches(&matches, &matches), GrepVerification::ExactMatch);
    /// ```
    pub fn verify_matches(
        &self,
        claimed: &[(usize, String)],
        ground_truth: &[(usize, String)],
    ) -> GrepVerification {
        let claimed_set: HashSet<(usize, String)> = claimed.iter().cloned().collect();
        let truth_set: HashSet<(usize, String)> = ground_truth.iter().cloned().collect();

        // Check for exact match (including order)
        if claimed == ground_truth {
            return GrepVerification::ExactMatch;
        }

        // Check for unordered match
        if claimed_set == truth_set {
            return GrepVerification::UnorderedMatch;
        }

        // Check for false positives (claimed but not in ground truth)
        let false_positives: Vec<_> = claimed
            .iter()
            .filter(|item| !truth_set.contains(item))
            .cloned()
            .collect();

        // Check for false negatives (in ground truth but not claimed)
        let false_negatives: Vec<_> = ground_truth
            .iter()
            .filter(|item| !claimed_set.contains(item))
            .cloned()
            .collect();

        if !false_positives.is_empty() && !false_negatives.is_empty() {
            // Both false positives and negatives - complete mismatch
            GrepVerification::Mismatch
        } else if !false_positives.is_empty() {
            GrepVerification::HasFalsePositives { false_positives }
        } else if !false_negatives.is_empty() {
            GrepVerification::HasFalseNegatives { false_negatives }
        } else {
            // Should not reach here, but treat as mismatch
            GrepVerification::Mismatch
        }
    }
}
