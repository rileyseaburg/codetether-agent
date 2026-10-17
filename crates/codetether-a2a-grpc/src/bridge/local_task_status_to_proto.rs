//! local task status to proto conversion.
use super::*;

pub fn local_task_status_to_proto(status: &local::TaskStatus) -> proto::TaskStatus {
    proto::TaskStatus {
        state: local_task_state_to_proto(status.state).into(),
        update: status.message.as_ref().map(local_message_to_proto),
        timestamp: status.timestamp.as_ref().and_then(|ts| {
            chrono::DateTime::parse_from_rfc3339(ts)
                .ok()
                .map(|dt| prost_types::Timestamp {
                    seconds: dt.timestamp(),
                    nanos: dt.timestamp_subsec_nanos() as i32,
                })
        }),
    }
}
