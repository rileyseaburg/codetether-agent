//! Shared types for incremental derivation helpers.

use crate::session::index::SummaryRange;

/// Produced summary for a dropped transcript gap.
#[derive(Debug, Clone)]
pub struct SummaryGap {
    /// Original transcript range covered by the summary.
    pub range: SummaryRange,
    /// Budget-bounded summary content.
    pub content: String,
}

/// What a message in the derived projection came from. Tracked
/// parallel to the message vector so `dropped_ranges` can be
/// recomputed after pairing repair and overflow clamping (issue #231
/// item 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageOrigin {
    /// Original transcript message at index `usize` in the pre-derivation
    /// clone.
    Clone(usize),
    /// Summary covering `range` in the pre-derivation clone.
    Summary(SummaryRange),
    /// Synthetic placeholder injected by pairing repair — represents no
    /// original message.
    Synthetic,
}

/// Build a contiguous `(start, end)` range list for every original
/// index in `0..origin_len` that no surviving message in `origins`
/// covers. A message covers an index when:
///
/// * It is a [`MessageOrigin::Clone(idx)`] with that exact index, or
/// * It is a [`MessageOrigin::Summary(range)`] whose `range` contains it.
///
/// Synthetic injections never cover any original index.
pub fn uncovered_ranges(origins: &[MessageOrigin], origin_len: usize) -> Vec<(usize, usize)> {
    let mut covered = vec![false; origin_len];
    for origin in origins {
        match origin {
            MessageOrigin::Clone(idx) => {
                if *idx < origin_len {
                    covered[*idx] = true;
                }
            }
            MessageOrigin::Summary(range) => {
                let end = range.end.min(origin_len);
                let start = range.start.min(end);
                covered[start..end].fill(true);
            }
            MessageOrigin::Synthetic => {}
        }
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i < origin_len {
        if !covered[i] {
            let start = i;
            while i < origin_len && !covered[i] {
                i += 1;
            }
            out.push((start, i));
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncovered_ranges_full_coverage_returns_empty() {
        let range = SummaryRange::new(0, 4).unwrap();
        let origins = vec![
            MessageOrigin::Summary(range),
            MessageOrigin::Clone(4),
            MessageOrigin::Clone(5),
        ];
        assert!(uncovered_ranges(&origins, 6).is_empty());
    }

    #[test]
    fn uncovered_ranges_isolates_dropped_gaps() {
        // Clones 0,1 survived; summary covered 2..4; clone 4 survived;
        // 5 was dropped by overflow clamp; 6 survived.
        let summary = SummaryRange::new(2, 4).unwrap();
        let origins = vec![
            MessageOrigin::Clone(0),
            MessageOrigin::Clone(1),
            MessageOrigin::Summary(summary),
            MessageOrigin::Clone(4),
            MessageOrigin::Clone(6),
        ];
        assert_eq!(uncovered_ranges(&origins, 7), vec![(5, 6)]);
    }

    #[test]
    fn uncovered_ranges_treats_synthetic_as_invisible() {
        let origins = vec![
            MessageOrigin::Clone(0),
            MessageOrigin::Synthetic,
            MessageOrigin::Clone(3),
        ];
        // Index 1 and 2 are both uncovered — synthetic doesn't help.
        assert_eq!(uncovered_ranges(&origins, 4), vec![(1, 3)]);
    }
}
