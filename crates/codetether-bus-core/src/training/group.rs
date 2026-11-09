//! Validate and assemble one correlated tool-call group.

use super::TrainingRecord;
mod validate;

/// Append a complete parallel tool-call group and reject invalid sequences.
pub(super) fn append(records: &mut [TrainingRecord], output: &mut Vec<TrainingRecord>) {
    if records.is_empty() {
        return;
    }
    let assistant_count = records.iter().filter(|record| is_request(record)).count();
    let tool_count = records.iter().filter(|record| is_response(record)).count();
    match validate::complete(records) {
        Ok(()) => {
            // Stable partition retains chronological order within each role.
            // The caller has already grouped by sender, step and correlation ID.
            records.sort_by_key(|record| !is_request(record));
            super::merge::merge(records, assistant_count, output);
        }
        Err(reason) => {
            tracing::warn!(
                records = records.len(),
                requests = assistant_count,
                responses = tool_count,
                reason,
                "Dropping incomplete tool group from training data"
            );
        }
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
