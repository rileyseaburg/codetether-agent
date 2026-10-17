//! File existence and size gate before loading cognition state.
use super::{LoadEvent, state::MAX_STATE_FILE_BYTES};
use std::path::Path;

pub(super) fn allow(path: &Path, report: &mut impl FnMut(LoadEvent<'_>)) -> bool {
    if !path.exists() {
        return false;
    }
    // Guard against bloated state files that would OOM the process
    match std::fs::metadata(path) {
        Ok(meta) if meta.len() > MAX_STATE_FILE_BYTES => {
            report(LoadEvent::TooLarge {
                size: meta.len(),
                max: MAX_STATE_FILE_BYTES,
            });
            let _ = std::fs::rename(path, path.with_extension("json.bloated"));
            return false;
        }
        Err(e) => {
            report(LoadEvent::StatFailed(&e));
            return false;
        }
        _ => {}
    }

    true
}
