//! Append or replace the CodeTether execution signature in a PR body.
//! [merge_signature] combines caller text with a rendered [SignatureContext].
use crate::context::SignatureContext;

const START: &str = "<!-- codetether-signature:start -->";

const END: &str = "<!-- codetether-signature:end -->";
mod merge_signature;
pub use merge_signature::merge_signature;
mod render_signature;
use render_signature::render_signature;
mod display;
use display::display;
mod push_line;
use push_line::push_line;
mod signature_range;
use signature_range::signature_range;
#[cfg(test)]
mod tests;
