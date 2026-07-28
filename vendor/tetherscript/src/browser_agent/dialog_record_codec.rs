//! Encode and decode stored dialog records.

use crate::browser_agent::dialog::dialog_codec::{escape, opt, parse_opt, unescape};
use crate::browser_agent::dialog::{DialogKind, DialogRecord};

pub(crate) fn format_record(record: &DialogRecord) -> String {
    format!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        record.sequence,
        record.kind.as_str(),
        escape(&record.message),
        opt(&record.default_value),
        bool_opt(record.accepted),
        opt(&record.response)
    )
}

pub(crate) fn parse_record(line: &str) -> Option<DialogRecord> {
    let parts = line.split('\t').collect::<Vec<_>>();
    Some(DialogRecord {
        sequence: parts.first()?.parse().ok()?,
        kind: parse_kind(parts.get(1)?)?,
        message: unescape(parts.get(2)?),
        default_value: parse_opt(parts.get(3)?),
        accepted: parse_bool(parts.get(4)?),
        response: parse_opt(parts.get(5)?),
    })
}

fn parse_kind(value: &str) -> Option<DialogKind> {
    match value {
        "alert" => Some(DialogKind::Alert),
        "confirm" => Some(DialogKind::Confirm),
        "prompt" => Some(DialogKind::Prompt),
        _ => None,
    }
}

fn bool_opt(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "1",
        Some(false) => "0",
        None => "-",
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}
