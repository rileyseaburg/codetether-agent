//! glob pattern for core file tools.
use super::*;
pub(super) fn resolve(root: &PathBuf, pattern: &str) -> (String, String) {
    // Determine the base directory from the pattern for WalkBuilder.
    // E.g. "src/**/*.rs" → walk "src/", "**/*.ts" → walk ".".
    let pattern_path = {
        let candidate = PathBuf::from(pattern);
        if candidate.is_absolute() {
            candidate
        } else {
            root.join(candidate)
        }
    };

    let (base_dir, match_pattern) = {
        let p = pattern_path.as_path();
        let mut prefix = std::path::PathBuf::new();
        let mut found_meta = false;
        for component in p.components() {
            let s = component.as_os_str().to_string_lossy();
            if !found_meta && glob::Pattern::escape(&s) == *s {
                prefix.push(component);
            } else {
                found_meta = true;
            }
        }
        let base = if prefix.as_os_str().is_empty() {
            ".".to_string()
        } else {
            prefix.display().to_string()
        };
        (base, pattern_path.display().to_string())
    };

    (base_dir, match_pattern)
}
