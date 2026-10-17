//! Format ordered materialized paths as a newline-delimited listing.
use std::path::PathBuf;

/// Render a listing of materialized paths, one per line.
pub(super) fn render_listing(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
