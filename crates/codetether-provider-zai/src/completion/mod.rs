//! Buffered completion execution and caller-specific response interpretation.
//!
//! The application entry point accepts its already-built request body. The
//! injected-client method retains its original response interpretation.
mod application;
mod content;
mod finish;
mod injected;
mod parse;
mod send;
mod usage;
pub use application::complete_application;
#[cfg(test)]
mod tests;
