//! Insert cached summaries into incremental derivations.

use crate::provider::{ContentPart, Message, Role};
use crate::session::ResidencyLevel;
use crate::session::index::SummaryRange;

use super::incremental_types::{MessageOrigin, SummaryGap};

/// Interleave summaries for dropped ranges with kept transcript messages.
///
/// Returns three parallel vectors: the derived messages, their
/// per-entry [`ResidencyLevel`], and a [`MessageOrigin`] sidecar so
/// later transformations (pairing repair, overflow clamp) can keep
/// `dropped_ranges` accurate.
pub fn interleave(
    source: &[Message],
    keep: &[bool],
    gaps: &[SummaryGap],
) -> (Vec<Message>, Vec<ResidencyLevel>, Vec<MessageOrigin>) {
    let mut messages = Vec::new();
    let mut levels = Vec::new();
    let mut origins = Vec::new();
    let mut gap_iter = gaps.iter().peekable();
    for (idx, message) in source.iter().enumerate() {
        while matches!(gap_iter.peek(), Some(g) if g.range.start == idx) {
            if let Some(gap) = gap_iter.next() {
                messages.push(summary_message(gap));
                levels.push(ResidencyLevel::Compressed);
                origins.push(MessageOrigin::Summary(gap.range));
            }
        }
        if keep[idx] {
            messages.push(message.clone());
            levels.push(ResidencyLevel::Full);
            origins.push(MessageOrigin::Clone(idx));
        }
    }
    (messages, levels, origins)
}

fn summary_message(gap: &SummaryGap) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: format!(
                "[SUMMARY of turns {}..{}]\n{}",
                gap.range.start, gap.range.end, gap.content
            ),
        }],
    }
}

/// Turn a tuple range into a typed summary range.
pub fn range_from_tuple(range: (usize, usize)) -> Option<SummaryRange> {
    SummaryRange::new(range.0, range.1)
}
