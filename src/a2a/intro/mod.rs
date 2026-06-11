//! Mesh introduction handling: detection, dedup ledger, sending.
//!
//! Splits the intro concern out of `spawn.rs`/`server.rs`:
//! - [`detect`] — classify inbound messages as intros (tag or legacy text)
//! - [`ledger`] — persistent "already introduced" set across restarts
//! - [`send`] — outbound tagged intro with ledger dedup
//! - [`reply`] — canned no-LLM acknowledgement for inbound intros

pub mod detect;
pub mod ledger;
pub mod reply;
pub mod send;

pub use detect::is_intro;
pub use send::send_intro;

#[cfg(test)]
#[path = "tests_fixtures.rs"]
pub mod tests_fixtures;

#[cfg(test)]
mod tests;
