//! Compact rendering of local identity and remote collaborators.

use serde_json::Value;

pub(super) fn format(entries: &[Value]) -> String {
    let self_name = entries.iter().find_map(|entry| {
        entry
            .get("self")
            .and_then(Value::as_bool)
            .filter(|value| *value)
            .and_then(|_| entry.get("name")?.as_str())
    });
    let peers = entries
        .iter()
        .filter(|entry| entry.get("self").and_then(Value::as_bool) != Some(true))
        .collect::<Vec<_>>();
    let names = peers
        .iter()
        .filter_map(|peer| peer.get("name").and_then(Value::as_str))
        .take(4)
        .map(|name| format!("@{name}"))
        .collect::<Vec<_>>();
    let suffix = (peers.len() > names.len()).then(|| format!(" +{}", peers.len() - names.len()));
    let noun = if peers.len() == 1 {
        "collaborator"
    } else {
        "collaborators"
    };
    let roster = format!(
        "{} {noun} ready: {}{}",
        peers.len(),
        names.join(", "),
        suffix.unwrap_or_default()
    );
    self_name
        .map(|name| format!("you: @{name}; {roster}"))
        .unwrap_or(roster)
}
