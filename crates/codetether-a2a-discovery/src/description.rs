//! Workspace-aware descriptions for automatically started A2A peers.

/// Resolve an explicit description or derive one from the workspace name.
///
/// # Arguments
/// * `explicit` - Description to preserve verbatim when supplied.
/// # Returns
/// The explicit description, or a description naming the current directory.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::description::resolve;
/// assert_eq!(resolve(Some("Voice API owner")), "Voice API owner");
/// ```
pub fn resolve(explicit: Option<&str>) -> String {
    explicit.map(ToString::to_string).unwrap_or_else(|| {
        let workspace = std::env::current_dir()
            .ok()
            .and_then(|path| path.file_name().map(|name| name.to_owned()))
            .and_then(|name| name.into_string().ok())
            .unwrap_or_else(|| "workspace".to_string());
        format!("CodeTether agent for workspace {workspace}")
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn preserves_an_explicit_description() {
        assert_eq!(super::resolve(Some("Voice API owner")), "Voice API owner");
    }
}
