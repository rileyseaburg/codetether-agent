//! Combine persisted and live sources; render only active constraints.
use super::*;
use std::collections::BTreeMap;

pub(super) fn pinned_constraints(
    prefix: Vec<ConstraintRecord>,
    pages: &[PageKind],
    source: &[Message],
    offset: &impl Fn() -> usize,
    corrections: &[ConstraintCorrection],
) -> Vec<String> {
    let mut records: BTreeMap<_, _> = prefix
        .into_iter()
        .map(|record| (record.source_turn, record))
        .collect();
    for (index, (page, message)) in pages.iter().zip(source).enumerate() {
        let turn = offset().saturating_add(index);
        if let Some(record) = ConstraintRecord::from_message(turn, message, *page) {
            records.insert(turn, record);
        } else {
            records.remove(&turn);
        }
    }
    records
        .values()
        .filter(|record| record.superseded_by.is_none())
        .map(|record| {
            let replaced = records
                .values()
                .filter(|old| old.superseded_by == Some(record.source_turn))
                .map(|old| old.source_turn.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "[source turn {}; role {:?}; sha256 {}; supersedes [{}]]\n{}",
                record.source_turn,
                record.source_role,
                record.text_hash,
                replaced,
                super::corrected_text::render(record, &records, corrections)
            )
        })
        .collect()
}
