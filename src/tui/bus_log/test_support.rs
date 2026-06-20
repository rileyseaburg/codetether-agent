//! Test helpers for bus log state tests.

use ratatui::style::Color;

use super::BusLogEntry;

pub(super) fn entry(summary: &str, topic: &str) -> BusLogEntry {
    BusLogEntry {
        timestamp: "00:00:00.000".to_string(),
        topic: topic.to_string(),
        sender_id: "tester".to_string(),
        kind: "MSG".to_string(),
        summary: summary.to_string(),
        detail: summary.to_string(),
        kind_color: Color::Cyan,
    }
}
