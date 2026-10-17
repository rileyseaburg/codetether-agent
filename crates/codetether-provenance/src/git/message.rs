//! Attach provenance trailers without duplicating existing labels.

use crate::{ExecutionProvenance, provenance_trailers};

pub fn commit_message_with_provenance(
    commit_msg: &str,
    provenance: Option<&ExecutionProvenance>,
) -> String {
    let Some(provenance) = provenance else {
        return commit_msg.to_string();
    };
    let trailers = build_trailers(commit_msg, provenance);
    if trailers.is_empty() {
        return commit_msg.to_string();
    }
    let trailer_lines = trailers
        .into_iter()
        .map(|(label, value)| format!("{label}: {value}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}\n\n{}", commit_msg.trim_end(), trailer_lines)
}

fn build_trailers(
    commit_msg: &str,
    provenance: &ExecutionProvenance,
) -> Vec<(&'static str, String)> {
    provenance_trailers(provenance)
        .into_iter()
        .filter(|(label, _)| !commit_msg.contains(label))
        .collect()
}
