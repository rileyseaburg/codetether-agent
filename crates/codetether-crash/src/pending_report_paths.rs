//! Crash reporting pending report paths.
use super::*;
pub(super) fn pending_report_paths(report_dir: &Path) -> Result<Vec<PathBuf>> {
    if !report_dir.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    for entry in std::fs::read_dir(report_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            paths.push(path);
        }
    }

    paths.sort_by_key(|path| {
        std::fs::metadata(path)
            .ok()
            .and_then(|meta| meta.modified().ok())
    });

    Ok(paths)
}
