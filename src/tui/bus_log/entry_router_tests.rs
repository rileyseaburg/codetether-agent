//! Regression tests for [`super::entry_router`].
//!
//! `BusMessage::UserPrompt` was previously routed to the agent entry
//! builder, which hit an `unreachable!` and crashed the TUI whenever a
//! user submitted a message while the bus log was receiving events.

use chrono::Utc;

use crate::bus::{BusEnvelope, BusMessage};

use super::BusLogEntry;

#[test]
fn user_prompt_routes_without_panicking() {
    let env = BusEnvelope {
        id: "1".to_string(),
        topic: "agent.build.user".to_string(),
        sender_id: "build".to_string(),
        correlation_id: None,
        timestamp: Utc::now(),
        message: BusMessage::UserPrompt {
            agent_id: "build".to_string(),
            text: "hello from the TUI".to_string(),
            workspace: "/tmp/ws".to_string(),
            session_id: "sess-1".to_string(),
        },
    };
    let entry = BusLogEntry::from_envelope(&env);
    assert_eq!(entry.kind, "USER");
    assert!(entry.summary.contains("hello from the TUI"));
    assert!(entry.detail.contains("sess-1"));
    assert!(entry.detail.contains("/tmp/ws"));
}
