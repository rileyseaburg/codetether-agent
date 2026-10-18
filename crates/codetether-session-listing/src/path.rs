//! Session snapshot path classification for directory scans.

use std::path::Path;

pub(super) fn is_session_json(path: &Path, index_filename: &str) -> bool {
    let name = path.file_name().and_then(|name| name.to_str());
    !is_metadata(name, index_filename) && path.extension().is_some_and(|ext| ext == "json")
}

fn is_metadata(name: Option<&str>, index_filename: &str) -> bool {
    name == Some(".listing_cache.json")
        || name == Some(index_filename)
        || name.is_some_and(|name| name.ends_with(".checkpoint.json"))
        || name.is_some_and(|name| name.ends_with(".legacy.json"))
}
