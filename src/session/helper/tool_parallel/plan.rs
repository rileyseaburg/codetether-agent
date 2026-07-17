//! Ordered scheduling plan for mixed tool-call batches.

use std::ops::Range;

/// One dispatch barrier in a model-issued tool-call batch.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Batch {
    /// Contiguous read-only calls that may share the execution gate.
    Parallel(Range<usize>),
    /// One call requiring exclusive, ordered execution.
    Single(usize),
}

pub(crate) fn build(
    calls: &[(String, String, serde_json::Value)],
    allow_parallel: bool,
) -> Vec<Batch> {
    let mut batches = Vec::new();
    let mut index = 0;
    while index < calls.len() {
        if !allow_parallel || !super::eligibility::is_eligible(&calls[index].1, &calls[index].2) {
            batches.push(Batch::Single(index));
            index += 1;
            continue;
        }
        let start = index;
        while index < calls.len()
            && super::eligibility::is_eligible(&calls[index].1, &calls[index].2)
        {
            index += 1;
        }
        if index - start > 1 {
            batches.push(Batch::Parallel(start..index));
        } else {
            batches.push(Batch::Single(start));
        }
    }
    batches
}
