use chrono::Utc;

use crate::bus::{BusEnvelope, BusMessage};
use crate::tui::bus_log_payload::{
    DETAIL_MAX_BYTES, FIELD_MAX_BYTES, KIND_MAX_BYTES, SUMMARY_MAX_BYTES,
};

use super::BusLogEntry;

#[test]
fn display_fields_are_strictly_byte_bounded() {
    let text = "🧠".repeat(10_000);
    let envelope = envelope(BusMessage::UserPrompt {
        agent_id: text.clone(),
        text: text.clone(),
        workspace: text.clone(),
        session_id: text.clone(),
    });

    let entry = BusLogEntry::from_envelope(&envelope);

    assert!(entry.topic.len() <= FIELD_MAX_BYTES);
    assert!(entry.sender_id.len() <= FIELD_MAX_BYTES);
    assert!(entry.kind.len() <= KIND_MAX_BYTES);
    assert!(entry.summary.len() <= SUMMARY_MAX_BYTES);
    assert!(entry.detail.len() <= DETAIL_MAX_BYTES);
    let BusMessage::UserPrompt { text: source, .. } = &envelope.message else {
        panic!("expected prompt");
    };
    assert_eq!(source, &text);
}

#[test]
fn speech_does_not_retain_two_full_content_copies() {
    let envelope = envelope(BusMessage::AgentSpeech {
        act: "inform".into(),
        from: "agent-a".into(),
        to: "agent-b".into(),
        conversation_id: "conversation".into(),
        content: "x".repeat(DETAIL_MAX_BYTES * 4),
    });

    let entry = BusLogEntry::from_envelope(&envelope);

    assert!(entry.summary.len() <= SUMMARY_MAX_BYTES);
    assert!(entry.detail.len() <= DETAIL_MAX_BYTES);
}

fn envelope(message: BusMessage) -> BusEnvelope {
    BusEnvelope {
        id: "id".into(),
        topic: "t".repeat(2_000),
        sender_id: "s".repeat(2_000),
        correlation_id: None,
        timestamp: Utc::now(),
        message,
    }
}
