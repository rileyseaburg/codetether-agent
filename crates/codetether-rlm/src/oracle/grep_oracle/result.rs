//! Outcomes of claimed and actual match comparison.

/// Result of grep oracle verification.
///
/// # Examples
/// ```
/// use codetether_rlm::{GrepVerification};
/// let result = GrepVerification::ExactMatch;
/// assert!(matches!(result, GrepVerification::ExactMatch));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum GrepVerification {
    /// Answer matches ground truth exactly.
    ExactMatch,
    /// Answer matches but in different order.
    UnorderedMatch,
    /// Answer is a subset of ground truth (partial).
    SubsetMatch { claimed: usize, actual: usize },
    /// Answer contains claims not in ground truth (false positives).
    HasFalsePositives {
        false_positives: Vec<(usize, String)>,
    },
    /// Answer is missing ground truth items (false negatives).
    HasFalseNegatives {
        false_negatives: Vec<(usize, String)>,
    },
    /// Answer is completely different.
    Mismatch,
    /// Could not infer pattern from query.
    CannotVerify { reason: String },
}
