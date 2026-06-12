//! Drop index entries whose backing session file is gone.

use std::collections::HashMap;
use std::path::Path;

use super::summary::SessionSummary;

pub(super) fn retain_existing(
    index: &mut HashMap<String, SessionSummary>,
    sessions_dir: &Path,
) {
    index.retain(|id, _| file_exists(sessions_dir, id));
}

fn file_exists(sessions_dir: &Path, id: &str) -> bool {
    sessions_dir.join(format!("{id}.json")).exists()
}
