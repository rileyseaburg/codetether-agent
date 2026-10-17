//! Combine persisted constraint lines with bounded live pin excerpts.
use super::*;

pub(super) fn pinned_constraints(
    prefix: Vec<String>,
    pages: &[PageKind],
    source: &[Message],
    offset: &impl Fn() -> usize,
) -> Vec<String> {
    prefix
        .into_iter()
        .chain(
            crate::pins::pinned_indices(pages, source)
                .into_iter()
                .filter_map(|idx| {
                    first_text_excerpt(&source[idx])
                        .map(|s| format!("- turn {}: {s}", offset() + idx))
                }),
        )
        .take(MAX_CONSTRAINTS)
        .collect()
}
