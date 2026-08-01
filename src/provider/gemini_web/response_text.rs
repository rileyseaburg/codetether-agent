//! Selection of the authoritative text candidate from Gemini wire frames.

#[path = "capture.rs"]
mod capture;
#[path = "response_text/collect.rs"]
pub mod collect;
#[path = "response_text/consensus.rs"]
pub mod consensus;
#[path = "response_text/frame.rs"]
pub mod frame;
#[path = "response_text/parts.rs"]
pub mod parts;
#[path = "response_text/placeholder.rs"]
pub mod placeholder;
#[path = "response_text/reasoning.rs"]
pub mod reasoning;
#[path = "response_text/ui_markup.rs"]
pub mod ui_markup;

pub use parts::content_parts;
pub use reasoning::split_reasoning;
pub use ui_markup::strip_ui_markup;

/// Returns the agreed answer candidate from a StreamGenerate body.
///
/// Slot `[4][0][1][0]` also carries drafts, titles, and leaked reasoning, so
/// position alone is not authoritative: a 40 KB tool-result turn was observed
/// returning unrelated content as the final candidate. Selection is therefore by
/// agreement across frames, not order. Card placeholders are skipped so an
/// internal rendering URL never displaces real content.
///
/// Returns an empty string when no candidate repeats, so the caller can retry
/// rather than surface a guess as the assistant's answer.
pub(super) fn latest(raw: &str) -> String {
    capture::record("stream", raw);
    consensus::select(&collect::candidates(raw)).unwrap_or_default()
}

#[cfg(test)]
#[path = "response_text_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "response_text/placeholder_tests.rs"]
mod placeholder_tests;

#[cfg(test)]
#[path = "response_text/leak_tests.rs"]
mod leak_tests;
