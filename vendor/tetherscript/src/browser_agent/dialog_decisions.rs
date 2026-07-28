//! Stored dialog decision helpers.

use std::collections::HashMap;

use crate::browser_agent::dialog::dialog_codec::{escape, parse_opt};
use crate::browser_agent::dialog::DialogDecision;

const DECISIONS_KEY: &str = "__browser_agent_dialog_decisions";

pub(crate) fn decisions(storage: &HashMap<String, String>) -> Vec<DialogDecision> {
    storage
        .get(DECISIONS_KEY)
        .map(|raw| raw.lines().filter_map(parse_decision).collect())
        .unwrap_or_default()
}

pub(crate) fn append_decision(storage: &mut HashMap<String, String>, decision: DialogDecision) {
    let mut items = decisions(storage);
    items.push(decision);
    write_decisions(storage, &items);
}

pub(crate) fn discard_decisions(storage: &mut HashMap<String, String>, consumed: usize) {
    let items = decisions(storage)
        .into_iter()
        .skip(consumed)
        .collect::<Vec<_>>();
    write_decisions(storage, &items);
}

fn write_decisions(storage: &mut HashMap<String, String>, decisions: &[DialogDecision]) {
    let raw = decisions
        .iter()
        .map(format_decision)
        .collect::<Vec<_>>()
        .join("\n");
    storage.insert(DECISIONS_KEY.into(), raw);
}

fn format_decision(decision: &DialogDecision) -> String {
    match decision {
        DialogDecision::Accept => "accept\t-".into(),
        DialogDecision::Dismiss => "dismiss\t-".into(),
        DialogDecision::Prompt(value) => format!("prompt\t+{}", escape(value)),
    }
}

fn parse_decision(line: &str) -> Option<DialogDecision> {
    let parts = line.split('\t').collect::<Vec<_>>();
    match *parts.first()? {
        "accept" => Some(DialogDecision::Accept),
        "dismiss" => Some(DialogDecision::Dismiss),
        "prompt" => parse_opt(parts.get(1)?).map(DialogDecision::Prompt),
        _ => None,
    }
}
