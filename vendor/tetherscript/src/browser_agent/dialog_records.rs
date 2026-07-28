//! Stored dialog record helpers.

use std::collections::HashMap;

use crate::browser_agent::dialog::dialog_record_codec::{format_record, parse_record};
use crate::browser_agent::dialog::DialogRecord;

const RECORDS_KEY: &str = "__browser_agent_dialog_records";

pub(crate) fn records(storage: &HashMap<String, String>) -> Vec<DialogRecord> {
    storage
        .get(RECORDS_KEY)
        .map(|raw| raw.lines().filter_map(parse_record).collect())
        .unwrap_or_default()
}

pub(crate) fn write_records(storage: &mut HashMap<String, String>, records: &[DialogRecord]) {
    let raw = records
        .iter()
        .map(format_record)
        .collect::<Vec<_>>()
        .join("\n");
    storage.insert(RECORDS_KEY.into(), raw);
}

pub(crate) fn clear_records(storage: &mut HashMap<String, String>) {
    storage.remove(RECORDS_KEY);
}
