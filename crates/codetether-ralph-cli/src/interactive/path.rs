//! Interactive Ralph path behavior.
use std::path::Path;

/// Resolve a user-provided PRD path against the working directory.
/// # Arguments
/// * `cwd` - Working directory used for relative paths.
/// * `arg` - Optional PRD path, defaulting to prd.json.
/// # Returns
/// The absolute input or the working directory joined with a relative input.
/// # Examples
/// ~~~
/// assert_eq!(codetether_ralph_cli::interactive::resolve_prd_path(std::path::Path::new("/work"), None), std::path::Path::new("/work/prd.json"));
/// ~~~
pub fn resolve_prd_path(cwd: &Path, arg: Option<&str>) -> std::path::PathBuf {
    let raw = arg.unwrap_or("prd.json");
    let path = Path::new(raw);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}
