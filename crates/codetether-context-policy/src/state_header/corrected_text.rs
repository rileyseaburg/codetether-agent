//! Render only validated replaced spans as inactive; retain every other byte.
use super::{ConstraintCorrection, ConstraintRecord};
use std::collections::BTreeMap;

pub(super) fn render(
    record: &ConstraintRecord,
    records: &BTreeMap<usize, ConstraintRecord>,
    corrections: &[ConstraintCorrection],
) -> String {
    let mut spans: Vec<_> = corrections
        .iter()
        .filter(|edit| edit.source_turn == record.source_turn)
        .collect();
    spans.sort_by_key(|edit| (edit.start, edit.end));
    let mut output = String::new();
    let mut cursor = 0;
    for edit in spans {
        let replacement = active_replacement(records, edit.replacement_turn);
        let valid = replacement.is_some()
            && records
                .get(&edit.replacement_turn)
                .is_some_and(|next| edit.validate(record, next).is_ok());
        if !valid || edit.start < cursor {
            // A broken edge must not suppress possibly still binding user text.
            return format!(
                "[CONSTRAINT_CORRECTION_UNRESOLVED: source, replacement or span changed; recover the correction before acting on conflicting requirements]\n{}",
                record.text
            );
        }
        output.push_str(&record.text[cursor..edit.start]);
        output.push_str(&format!(
            "[superseded bytes {}..{} by user source turn {}]",
            edit.start,
            edit.end,
            replacement.unwrap()
        ));
        cursor = edit.end;
    }
    output.push_str(&record.text[cursor..]);
    output
}

fn active_replacement(
    records: &BTreeMap<usize, ConstraintRecord>,
    mut turn: usize,
) -> Option<usize> {
    for _ in 0..=records.len() {
        let record = records.get(&turn)?;
        if let Some(next) = record.superseded_by {
            if next <= turn {
                return None;
            }
            turn = next;
        } else {
            return Some(turn);
        }
    }
    None
}
