//! Classification and ordering helpers for S3 training records.

use super::TrainingRecord;
use crate::bus::BusMessage;
use chrono::DateTime;

pub(super) type ToolGroupKey = (String, usize, Option<String>);

pub(super) fn excluded(message: &BusMessage) -> bool {
    matches!(
        message,
        BusMessage::Heartbeat { .. } | BusMessage::ToolOutputFull { .. }
    )
}

pub(super) fn tool_group_key(
    record: &TrainingRecord,
    message: &BusMessage,
) -> Option<ToolGroupKey> {
    let step = record.metadata.step?;
    matches!(
        message,
        BusMessage::ToolRequest { .. } | BusMessage::ToolResponse { .. }
    )
    .then(|| {
        (
            record.metadata.sender_id.clone(),
            step,
            record.metadata.correlation_id.clone(),
        )
    })
}

pub(super) fn timestamp_key(record: &TrainingRecord) -> (i64, u32) {
    DateTime::parse_from_rfc3339(&record.metadata.timestamp)
        .map(|value| (value.timestamp(), value.timestamp_subsec_nanos()))
        .unwrap_or_default()
}
