//! Order and filter bus envelopes for supervised training output.

use super::s3_training_group::append;
use super::s3_training_order::{ToolGroupKey, excluded, timestamp_key, tool_group_key};
use super::{TrainingRecord, envelope_to_training_record};
use crate::bus::BusEnvelope;
use std::collections::BTreeMap;

#[cfg(test)]
#[path = "s3_training_order_tests.rs"]
mod order_tests;
#[cfg(test)]
#[path = "s3_training_collect_tests.rs"]
mod tests;

/// Convert envelopes into chronological, non-duplicated training records.
pub(super) fn collect(envelopes: &[BusEnvelope]) -> Vec<TrainingRecord> {
    let mut groups: BTreeMap<ToolGroupKey, Vec<TrainingRecord>> = BTreeMap::new();
    let mut output = Vec::new();
    for envelope in envelopes {
        if excluded(&envelope.message) {
            continue;
        }
        let record = envelope_to_training_record(envelope);
        if let Some(key) = tool_group_key(&record, &envelope.message) {
            groups.entry(key).or_default().push(record);
        } else {
            output.push(record);
        }
    }
    for mut group in groups.into_values() {
        group.sort_by_key(timestamp_key);
        append(&mut group, &mut output);
    }
    output.sort_by_key(timestamp_key);
    output
}
