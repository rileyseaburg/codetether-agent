//! Minimum fidelity and permitted page degradation paths.
use super::*;

impl PageKind {
    /// The minimum-fidelity level this kind is allowed to drop to.
    ///
    /// Compression / eviction strategies that would drop a page below
    /// its minimum fidelity must be **rejected** (ClawVM §3), not
    /// silently applied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_context_policy::pages::{PageKind, ResidencyLevel};
    ///
    /// assert_eq!(PageKind::Constraint.min_fidelity(), ResidencyLevel::Structured);
    /// assert_eq!(PageKind::Evidence.min_fidelity(), ResidencyLevel::Pointer);
    /// ```
    pub fn min_fidelity(self) -> ResidencyLevel {
        match self {
            PageKind::Bootstrap => ResidencyLevel::Structured,
            PageKind::Constraint => ResidencyLevel::Structured,
            PageKind::Plan => ResidencyLevel::Pointer,
            PageKind::Preference => ResidencyLevel::Pointer,
            PageKind::Evidence => ResidencyLevel::Pointer,
            PageKind::Conversation => ResidencyLevel::Pointer,
        }
    }

    /// The ordered sequence of residency levels this kind is allowed
    /// to visit, from full down to its minimum fidelity.
    ///
    /// Used by Phase B's `DerivePolicy::Incremental` to pick the next
    /// marginal downgrade when the token budget is tight.
    pub fn degradation_path(self) -> &'static [ResidencyLevel] {
        match self {
            PageKind::Bootstrap | PageKind::Constraint => {
                &[ResidencyLevel::Full, ResidencyLevel::Structured]
            }
            PageKind::Plan => &[
                ResidencyLevel::Full,
                ResidencyLevel::Structured,
                ResidencyLevel::Pointer,
            ],
            PageKind::Preference | PageKind::Evidence | PageKind::Conversation => &[
                ResidencyLevel::Full,
                ResidencyLevel::Compressed,
                ResidencyLevel::Structured,
                ResidencyLevel::Pointer,
            ],
        }
    }
}
