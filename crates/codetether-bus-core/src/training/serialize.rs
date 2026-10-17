//! Stable JSONL record encoding for a bus batch.
use super::*;

pub(super) fn serialize_training_records(records: &[TrainingRecord]) -> Vec<String> {
    records
        .iter()
        .filter_map(|record| serde_json::to_string(record).ok())
        .collect()
}
