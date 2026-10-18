//! Mesh introduction handling: detection, dedup ledger, sending.
//!
//! Splits the intro concern out of `spawn.rs`/`server.rs`:
//! - [`detect`] — classify inbound messages as intros (tag or legacy text)
//! - [`ledger`] — persistent "already introduced" set across restarts
//! - [`send`] — outbound tagged intro with ledger dedup
//! - [`reply`] — canned no-LLM acknowledgement for inbound intros

pub use codetether_a2a_http::intro::application::ledger;
pub use codetether_a2a_http::intro::application::send;
pub use codetether_a2a_http::intro::detect;
pub use codetether_a2a_http::intro::reply;

pub use detect::is_intro;
pub use send::send_intro;
