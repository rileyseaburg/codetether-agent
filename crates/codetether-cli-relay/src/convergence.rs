//! Stable comparison used for consecutive relay handoffs.

pub(super) fn normalize_for_convergence(text: &str) -> String {
    codetether_autochat_core::normalize_for_convergence(text, 280)
}
