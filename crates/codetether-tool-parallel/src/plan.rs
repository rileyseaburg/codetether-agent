//! Ordered scheduling plan for mixed tool-call batches.

use std::ops::Range;

/// One dispatch barrier in a model-issued tool-call batch.
///
/// # Examples
/// ```
/// use codetether_tool_parallel::Batch;
/// match Batch::Single(0) {
///     Batch::Single(index) => assert_eq!(index, 0),
///     Batch::Parallel(range) => assert!(!range.is_empty()),
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Batch {
    /// Contiguous read-only calls that may share the execution gate.
    Parallel(Range<usize>),
    /// One call requiring exclusive, ordered execution.
    Single(usize),
}

/// Group adjacent eligible reads while preserving exclusive call barriers.
/// # Arguments
/// `calls` contains IDs, tool names and input; `allow_parallel` enables grouping.
/// # Returns
/// An ordered plan covering each input call exactly once.
/// # Examples
/// ```
/// let calls = vec![("1".into(), "read".into(), serde_json::json!({}))];
/// assert_eq!(codetether_tool_parallel::plan(&calls, true),
///            vec![codetether_tool_parallel::Batch::Single(0)]);
/// ```
pub fn build(calls: &[(String, String, serde_json::Value)], allow_parallel: bool) -> Vec<Batch> {
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
