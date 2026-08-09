//! Policy for whether a symbol-search keystroke should issue an LSP query.
//!
//! Pure logic, so it is unit-testable without a language server. Keeping this
//! separate from [`super::refresh_symbol_search`] means the expensive I/O path
//! has one decision point instead of scattered guards.

use std::time::Duration;

/// Queries shorter than this match nearly every symbol and are the slowest.
pub(super) const MIN_QUERY_LEN: usize = 2;

/// Rapid keystrokes inside this window coalesce into a single query.
pub(super) const DEBOUNCE: Duration = Duration::from_millis(150);

/// Whether `query` is specific enough to send to the language server.
pub(super) fn is_searchable(query: &str) -> bool {
    query.chars().count() >= MIN_QUERY_LEN
}

#[cfg(test)]
#[path = "gate_tests.rs"]
mod tests;
