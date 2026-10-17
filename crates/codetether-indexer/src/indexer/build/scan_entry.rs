//! scan entry for index construction.

use super::*;

pub(super) fn scan_entry(
    entry: DirEntry,
    root: &Path,
    options: &BuildOptions,
    data: &mut IndexInputs,
) {
    let path = entry.path();
    if !path.is_file() {
        return;
    }

    data.stats.total_seen_files += 1;

    let rel_path = path.strip_prefix(root).unwrap_or(path);

    if !options.include_hidden && is_hidden_path(rel_path) {
        data.stats.skipped_hidden += 1;
        return;
    }

    let Some((metadata, content)) = read_input(path, options, &mut data.stats) else {
        return;
    };
    record_file(path, rel_path, &metadata, &content, options, data);
}
