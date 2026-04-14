use std::collections::HashSet;

const DEFAULT_PUBLIC_PATHS: &[&str] = &["/health"];

/// Build the default set of unauthenticated paths.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::server::auth::default_public_paths;
///
/// let paths = default_public_paths();
/// assert!(paths.contains("/health"));
/// ```
pub fn default_public_paths() -> HashSet<String> {
    DEFAULT_PUBLIC_PATHS
        .iter()
        .map(|path| (*path).to_string())
        .collect()
}
