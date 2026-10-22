//! Rebase source links to the exact retained fork window.
use crate::ForkSession;
use codetether_context_policy::{pages::PageKind, state_header::ConstraintCorrection};

pub(super) fn retained<S: ForkSession>(
    source: &S,
    start: usize,
    end: usize,
) -> (Vec<PageKind>, Vec<ConstraintCorrection>) {
    let first = source.message_offset().saturating_add(start);
    let last = source.message_offset().saturating_add(end);
    let mut pages = source
        .page_kinds()
        .get(start..end)
        .map(<[PageKind]>::to_vec)
        .unwrap_or_else(|| {
            codetether_context_policy::pages::classify_all(&source.messages()[start..end])
        });
    for page in &mut pages {
        if let PageKind::SupersededConstraint { by_turn } = *page {
            // A fork before a correction restores the prior instruction.
            *page = if (first..last).contains(&by_turn) {
                PageKind::SupersededConstraint {
                    by_turn: by_turn - first,
                }
            } else {
                PageKind::Constraint
            };
        }
    }
    let corrections = source
        .metadata()
        .constraint_corrections
        .iter()
        .filter(|edit| {
            (first..last).contains(&edit.source_turn)
                && (first..last).contains(&edit.replacement_turn)
        })
        .cloned()
        .map(|mut edit| {
            edit.source_turn -= first;
            edit.replacement_turn -= first;
            edit
        })
        .collect();
    (pages, corrections)
}
