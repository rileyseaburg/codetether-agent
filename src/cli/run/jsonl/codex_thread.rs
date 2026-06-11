//! Codex-shaped JSONL adapter for durable thread events.

use std::io::Write;

use anyhow::Result;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serialize;
use serde_json::{Map, Value};

use crate::session::thread_store::ThreadEvent;

#[derive(Serialize)]
struct CodexThreadRecord {
    timestamp: String,
    #[serde(rename = "type")]
    kind: String,
    payload: Value,
}

pub(in crate::cli::run) fn write_codex_thread_event_to<W: Write>(
    mut writer: W,
    event: &ThreadEvent,
) -> Result<bool> {
    serde_json::to_writer(&mut writer, &record(event))?;
    writeln!(writer)?;
    Ok(true)
}

fn record(event: &ThreadEvent) -> CodexThreadRecord {
    CodexThreadRecord {
        timestamp: timestamp(event.timestamp_ms),
        kind: event.kind.clone(),
        payload: payload(event),
    }
}

fn payload(event: &ThreadEvent) -> Value {
    let mut object = match event.payload.clone() {
        Value::Object(object) => object,
        _ => Map::new(),
    };
    object.insert("event_id".into(), Value::String(event.event_id.clone()));
    object.insert("thread_id".into(), Value::String(event.thread_id.clone()));
    object.insert("session_id".into(), Value::String(event.session_id.clone()));
    object.insert("turn_id".into(), Value::String(event.turn_id.clone()));
    Value::Object(object)
}

fn timestamp(timestamp_ms: u64) -> String {
    let millis = i64::try_from(timestamp_ms).unwrap_or(i64::MAX);
    DateTime::<Utc>::from_timestamp_millis(millis)
        .unwrap_or_else(|| DateTime::<Utc>::from(std::time::UNIX_EPOCH))
        .to_rfc3339_opts(SecondsFormat::Millis, true)
}
