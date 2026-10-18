//! Assistant output collection, streaming and final-answer policy.
//!
//! [emit] preserves stream-event order, publishes assistant bus messages, and
//! suppresses [narration] from the final answer while keeping it visible live.
mod collect;
mod context;
mod emit;
pub mod narration;
pub use context::Context;
pub use emit::emit;

mod stream;
