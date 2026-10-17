//! Shared memory retrieval exercised by application-store regression tests.
pub use codetether_memory_core::search::run;
#[cfg(test)]
#[path = "search_tests.rs"]
mod tests;
