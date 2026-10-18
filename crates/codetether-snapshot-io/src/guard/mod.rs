//! Remember successful hashes without trusting them over current file contents.
mod hash;
mod query;
mod record;
mod state;
pub use query::is_unchanged;
pub use record::{forget, record_saved};
#[cfg(test)]
mod tests;
