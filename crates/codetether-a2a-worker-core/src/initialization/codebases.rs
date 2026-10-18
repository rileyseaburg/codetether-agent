//! Parse explicit workspace lists or use the current directory.
/// Resolve startup workspace strings in their configured order.
/// # Arguments
/// `workspaces` is the optional comma-separated CLI value.
/// # Returns
/// Trimmed entries, or one current-directory entry when not configured.
/// # Panics
/// Panics when no value is configured and the current directory cannot be read.
/// # Examples
/// ```
/// assert_eq!(codetether_a2a_worker_core::initialization::parse_codebases(Some(" a , b ")), vec!["a", "b"]);
/// ```
pub fn parse_codebases(workspaces: Option<&str>) -> Vec<String> {
    workspaces
        .map(|items| {
            items
                .split(',')
                .map(|item| item.trim().to_string())
                .collect()
        })
        .unwrap_or_else(|| vec![std::env::current_dir().unwrap().display().to_string()])
}
