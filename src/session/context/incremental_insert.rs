//! Insert cached summaries into incremental derivations.

use crate::provider::{ContentPart, Message, Role};
use crate::session::ResidencyLevel;
use crate::session::index::SummaryRange;

use super::incremental_types::SummaryGap;

/// Interleave summaries for dropped ranges with kept transcript messages.
pub fn interleave(
    source: &[Message],
    keep: &[bool],
    gaps: &[SummaryGap],
) -> (Vec<Message>, Vec<ResidencyLevel>) {
    let mut messages = Vec::new();
    let mut levels = Vec::new();
    let mut gap_iter = gaps.iter().peekable();
    for (idx, message) in source.iter().enumerate() {
        while matches!(gap_iter.peek(), Some(g) if g.range.start == idx) {
            if let Some(gap) = gap_iter.next() {
                messages.push(summary_message(gap));
                levels.push(ResidencyLevel::Compressed);
            }
        }
        if keep[idx] {
            messages.push(message.clone());
            levels.push(ResidencyLevel::Full);
        }
    }
    (messages, levels)
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
