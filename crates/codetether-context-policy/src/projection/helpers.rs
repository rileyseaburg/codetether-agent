//! Derived-context exports and message-count comparison.
pub use super::derived_context::DerivedContext;
#[cfg(test)]
use crate::pages::ResidencyLevel;
use codetether_provider_core::Message;

/// Compare two message counts and return whether compression fired.
///
/// Separated out so the comparison is testable without a full provider
/// round-trip. Any count change is treated as evidence that compression
/// fired.
pub fn messages_len_changed(before: usize, after: &[Message]) -> bool {
    before != after.len()
}
#[cfg(test)]
#[path = "helper_tests.rs"]
mod tests;
