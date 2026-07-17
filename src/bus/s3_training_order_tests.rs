//! Regression test for chronological training-record ordering.

use super::*;
use crate::bus::BusMessage;
use chrono::{TimeZone, Utc};

fn envelope(second: u32, kind: &str) -> BusEnvelope {
    BusEnvelope {
        id: kind.into(),
        topic: "agent.build".into(),
        sender_id: "build".into(),
        correlation_id: Some("turn-1".into()),
        timestamp: Utc.with_ymd_and_hms(2026, 7, 17, 20, 7, second).unwrap(),
        message: BusMessage::AgentThinking {
            agent_id: "build".into(),
            thinking: kind.into(),
            step: second as usize,
        },
    }
}

#[test]
fn sorts_passthrough_records_by_timestamp() {
    let records = collect(&[envelope(2, "later"), envelope(1, "earlier")]);
    assert_eq!(
        records[0].content.as_deref(),
        Some("<thinking>\nearlier\n</thinking>")
    );
    assert_eq!(
        records[1].content.as_deref(),
        Some("<thinking>\nlater\n</thinking>")
    );
}
