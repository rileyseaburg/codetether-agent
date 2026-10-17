//! Merge parallel assistant tool requests into one chat message.

use super::TrainingRecord;

pub(super) fn merge(
    records: &[TrainingRecord],
    assistant_count: usize,
    output: &mut Vec<TrainingRecord>,
) {
    let mut merged = records[0].clone();
    let mut tool_calls = Vec::with_capacity(assistant_count);
    let mut envelope_ids = Vec::with_capacity(assistant_count);
    let mut contents = Vec::new();
    for record in records.iter().take(assistant_count) {
        envelope_ids.push(record.metadata.envelope_id.clone());
        if let Some(content) = record
            .content
            .as_ref()
            .filter(|content| !content.is_empty())
        {
            contents.push(content.clone());
        }
        if let Some(mut calls) = record.tool_calls.clone() {
            tool_calls.append(&mut calls);
        }
    }
    merged.tool_calls = Some(tool_calls);
    merged.content = (!contents.is_empty()).then(|| contents.join("\n"));
    merged.metadata.bus_kind = "tool_request_batch".into();
    merged.metadata.envelope_id = envelope_ids.join(",");
    output.push(merged);
    output.extend(records.iter().skip(assistant_count).cloned());
}
