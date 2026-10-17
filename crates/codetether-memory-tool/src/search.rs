//! Shared memory retrieval and store-level regression tests.
pub use codetether_memory_core::search::run;
#[cfg(test)]
#[path = "search_tests.rs"]
mod tests;
