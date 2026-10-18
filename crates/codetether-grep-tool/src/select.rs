//! select for bounded grep searches.
pub(super) fn entry(
    entry: std::result::Result<ignore::DirEntry, ignore::Error>,
    include_pattern: &Option<glob::Pattern>,
) -> Option<ignore::DirEntry> {
    let entry = match entry {
        Ok(e) => e,
        Err(_) => return None,
    };

    if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
        return None;
    }

    let path = entry.path();

    // Check include pattern
    if let Some(include_pattern) = &include_pattern
        && !include_pattern.matches_path(path)
    {
        return None;
    }

    Some(entry)
}
