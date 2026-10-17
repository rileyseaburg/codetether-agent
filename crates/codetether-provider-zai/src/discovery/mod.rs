//! Discovery transports and model decoding for the existing clients.
//!
//! Application discovery preserves trimmed IDs and server display names.
//! Injected-client discovery retains its raw-ID and vision inference contract.
mod application;
mod application_model;
mod fetch;
mod injected;
mod injected_model;
pub use application::discover_application;
#[cfg(test)]
mod tests;
