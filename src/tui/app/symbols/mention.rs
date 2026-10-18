//! Application composer binding for shared symbol mention behavior.
mod host;
pub use codetether_symbol_mention::{confirm, dismiss, open, starts_mention};
#[cfg(test)]
mod tests;
