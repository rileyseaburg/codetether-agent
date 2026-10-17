//! Reconstruct a reset projection from the supplied summary and retained tail.
use super::*;
/// Build the summary message, append the active tail and repair pairing.
///
/// # Arguments
/// * `summary` - Text produced for the discarded prefix.
/// * `toc` - Host-rendered index of that prefix.
/// * `tail` - Active task messages retained verbatim.
/// * `origin_len` - Original transcript length.
///
/// # Returns
/// A compressed projection with repaired tool pairing.
///
/// # Examples
/// ```
/// use codetether_context_policy::derivation::reset::rebuild;
/// assert!(rebuild("summary", "", &[], 3).compressed);
/// ```
pub fn rebuild(summary: &str, toc: &str, tail: &[Message], origin_len: usize) -> DerivedContext {
    let mut reset_messages = Vec::with_capacity(1 + tail.len());
    reset_messages.push(markers::build_reset_summary_message(summary, toc));
    reset_messages.extend_from_slice(tail);

    pairing::repair_orphans(&mut reset_messages);

    DerivedContext {
        resolutions: vec![ResidencyLevel::Full; reset_messages.len()],
        dropped_ranges: Vec::new(),
        provenance: vec!["reset".to_string(), "rlm_summary".to_string()],
        messages: reset_messages,
        origin_len,
        compressed: true,
    }
}
