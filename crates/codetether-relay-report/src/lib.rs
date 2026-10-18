//! CLI relay result schema and human-readable summary formatting.
//!
//! [AutochatCliResult] retains the serialized output contract. [summary] renders
//! final relay facts without depending on sessions, providers or bus runtime.
mod counters;
mod lead;
mod result;
mod summary;
pub use result::AutochatCliResult;
pub use summary::Summary;
/// Render the original relay status, handoff and optional counter messages.
/// # Arguments
/// * `input` - Final relay facts.
/// # Returns
/// Human-readable summary with a bounded handoff excerpt.
/// # Examples
/// See [Summary] for a complete construction example.
pub fn summary(input: &Summary<'_>) -> String {
    let mut summary = lead::build(input);
    counters::append(&mut summary, input);
    summary
}
