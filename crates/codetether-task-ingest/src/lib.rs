//! CloudEvent adaptation and in-process task ingestion.
//!
//! receive_task_event audits and dispatches task events against caller-owned
//! queue/log instances. parse_cloud_event adapts HTTP headers to the shared parser.
mod cancelled;
mod created;
mod dispatch;
mod fields;
pub mod http;
mod parse;
mod receive;
mod updated;
pub use parse::{CloudEvent, parse_cloud_event};
pub use receive::receive_task_event;
use serde::Serialize;

/// Response to CloudEvent acknowledgment
#[derive(Serialize)]
pub struct CloudEventResponse {
    status: String,
    event_id: String,
}
