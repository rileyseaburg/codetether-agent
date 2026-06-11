//! Helpers for grouping parsed hunks by target file.

use super::types::PatchHunk;
use std::collections::{BTreeMap, BTreeSet};

/// Return unique target files in stable order.
pub(super) fn files(hunks: &[PatchHunk]) -> Vec<String> {
    hunks
        .iter()
        .map(|hunk| hunk.file.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Group hunks by target file in stable order.
pub(super) fn by_file(hunks: &[PatchHunk]) -> BTreeMap<String, Vec<&PatchHunk>> {
    let mut by_file: BTreeMap<String, Vec<&PatchHunk>> = BTreeMap::new();
    for hunk in hunks {
        by_file.entry(hunk.file.clone()).or_default().push(hunk);
    }
    by_file
}
