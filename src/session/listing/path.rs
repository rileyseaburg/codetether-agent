//! Session snapshot path classification for directory scans.

use std::path::Path;

use crate::session::workspace_index::INDEX_FILENAME;

pub(super) fn is_session_json(path: &Path) -> bool {
    let name = path.file_name().and_then(|name| name.to_str());
    !is_metadata(name) && path.extension().is_some_and(|ext| ext == "json")
}

fn is_metadata(name: Option<&str>) -> bool {
    name == Some(".listing_cache.json")
        || name == Some(INDEX_FILENAME)
        || name.is_some_and(|name| name.ends_with(".checkpoint.json"))
}
