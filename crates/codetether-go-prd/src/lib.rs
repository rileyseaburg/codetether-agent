//! PRD generation, response recovery and provider retries.
//!
//! Use [`generate_prd_from_task`] with a configured provider or
//! [`generate_prd_with_model_retry`] with an optional [`Registry`]. Requests
//! produce shared Ralph PRDs; this crate does not execute their quality commands.

mod alternatives;
mod balanced;
mod candidates;
mod commas;
mod comments;
mod extract;
mod generate;
mod mismatch;
mod normalize;
mod parse;
mod prompt;
mod quality;
mod registry;
mod repair_request;
mod repair_text;
mod request;
mod response_text;
mod retry;
mod sanitize;
pub use generate::generate_prd_from_task;
pub use registry::Registry;
pub use retry::generate_prd_with_model_retry;
#[cfg(test)]
mod tests;
