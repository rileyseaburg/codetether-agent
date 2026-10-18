//! Assistant reasoning and visible-text publication on the agent bus.
//!
//! [thinking] emits compact reasoning with the current step. [text] emits the
//! visible completion addressed to the user. Both preserve turn correlation.
mod send;
mod text;
mod thinking;
pub use text::text;
pub use thinking::thinking;
