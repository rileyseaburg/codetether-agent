//! Workspace scanning for the fuzzy file finder.
//!
//! Walks the workspace honoring `.gitignore` (via the `ignore` crate),
//! collecting regular files as [`FuzzyEntry`] candidates with
//! workspace-relative display strings.

use std::path::Path;

use ignore::WalkBuilder;

use super::state::FuzzyEntry;

/// Maximum number of files scanned to keep the finder responsive.
const MAX_CANDIDATES: usize = 20_000;

/// Scans `root` for files, returning ranked-display candidates.
pub fn scan_workspace(root: &Path) -> Vec<FuzzyEntry> {
    let mut out = Vec::new();
    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .build();
    for entry in walker.flatten() {
        if out.len() >= MAX_CANDIDATES {
            break;
        }
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            let path = entry.path().to_path_buf();
            let display = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            out.push(FuzzyEntry { display, path });
        }
    }
    out.sort_by(|a, b| a.display.cmp(&b.display));
    out
}
