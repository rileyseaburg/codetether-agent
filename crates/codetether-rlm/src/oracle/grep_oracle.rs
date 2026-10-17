//! Pattern inference, source matching, and answer comparison.

mod classify;
mod extract;
mod format;
mod infer;
mod literal;
mod matches;
mod patterns;
mod result;
mod source;
mod verify;
pub use result::GrepVerification;
pub use source::GrepOracle;
#[cfg(test)]
mod tests;
