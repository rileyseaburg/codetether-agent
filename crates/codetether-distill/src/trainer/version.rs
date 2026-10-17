//! Select the next adapter version from existing entry names.
use std::path::Path;
pub(super) fn next_version(output_dir: &Path) -> usize {
    std::fs::read_dir(output_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    e.file_name()
                        .to_str()?
                        .strip_prefix('v')?
                        .parse::<usize>()
                        .ok()
                })
                .max()
                .unwrap_or(0)
                + 1
        })
        .unwrap_or(1)
}
