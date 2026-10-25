//! Gemini Web backend orchestration, credentials, and versioned local artifacts.
//!
//! The transport and protocol are native Rust. Each generated version has its
//! own directory and optional private continuation state for later refinements.
mod artifacts;
mod client;
mod cookie_file;
mod credentials;
pub(super) mod generate;
mod private_files;
mod result;
mod state;
#[cfg(test)]
mod tests;
pub(super) mod validate;
