//! read input for index construction.

use super::*;

pub(super) fn read_input(
    path: &Path,
    options: &BuildOptions,
    stats: &mut IndexStats,
) -> Option<(std::fs::Metadata, String)> {
    let metadata = match std::fs::metadata(path) {
        Ok(meta) => meta,
        Err(_) => {
            stats.skipped_io_errors += 1;
            return None;
        }
    };

    if metadata.len() > options.max_file_size_bytes {
        stats.skipped_large += 1;
        return None;
    }

    if !is_probably_text_file(path) {
        stats.skipped_non_text += 1;
        return None;
    }

    let content = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => {
            stats.skipped_non_text += 1;
            return None;
        }
    };

    Some((metadata, content))
}
