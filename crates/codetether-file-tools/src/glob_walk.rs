//! glob walk for core file tools.
use super::*;
pub(super) fn collect(
    root: &PathBuf,
    base_dir: &str,
    compiled: &glob::Pattern,
    limit: usize,
    matches: &mut Vec<String>,
) {
    let walker = ignore::WalkBuilder::new(&base_dir)
        .hidden(false)
        .git_ignore(true)
        .follow_links(false) // prevent symlink loops
        .max_depth(Some(30)) // hard depth cap
        .build();

    for entry in walker {
        if matches.len() >= limit {
            break;
        }
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if compiled.matches_path(path) || compiled.matches(path.to_string_lossy().as_ref()) {
            let display = path
                .strip_prefix(root)
                .unwrap_or(path)
                .display()
                .to_string();
            matches.push(display);
        }
    }
}
