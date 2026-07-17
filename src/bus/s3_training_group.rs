//! Validate and assemble one correlated tool-call group.

use super::TrainingRecord;
use std::collections::BTreeSet;

/// Append a complete parallel tool-call group and reject invalid sequences.
pub(super) fn append(records: &mut [TrainingRecord], output: &mut Vec<TrainingRecord>) {
    if records.is_empty() {
        return;
    }
    let assistant_count = records
        .iter()
        .take_while(|record| is_request(record))
        .count();
    let tool_count = records[assistant_count..]
        .iter()
        .take_while(|record| is_response(record))
        .count();
    let complete = assistant_count > 0
        && tool_count > 0
        && assistant_count + tool_count == records.len()
        && assistant_count == tool_count
        && ids_match(records, assistant_count);
    if complete {
        super::s3_training_merge::merge(records, assistant_count, output);
    } else {
        tracing::warn!(
            records = records.len(),
            "Dropping incomplete tool group from training data"
        );
    }
}

fn is_request(record: &TrainingRecord) -> bool {
    record.role == "assistant"
        && record
            .tool_calls
            .as_ref()
            .is_some_and(|calls| calls.len() == 1)
}

fn is_response(record: &TrainingRecord) -> bool {
    record.role == "tool" && record.tool_call_id.is_some()
}

fn ids_match(records: &[TrainingRecord], assistant_count: usize) -> bool {
    let requests: BTreeSet<_> = records[..assistant_count]
        .iter()
        .filter_map(|record| record.tool_calls.as_ref()?.first())
        .map(|call| call.id.as_str())
        .collect();
    let responses: BTreeSet<_> = records[assistant_count..]
        .iter()
        .filter_map(|record| record.tool_call_id.as_deref())
        .collect();
    requests == responses && requests.len() == assistant_count
}
