//! Derive an ephemeral legacy context without mutating stored history.
//!
//! [derive_context] clones a [Source] transcript, applies configured [Hooks],
//! compresses it to budget and returns provenance with the derived messages.
mod compress_step;
mod derive;
mod finish;
mod forced;
mod hooks;
mod source;
pub use derive::derive_context;
pub use hooks::Hooks;
pub use source::Source;

mod prepare;
