use std::path::Path;

use anyhow::Result;

pub fn plugin_path(candidate: &Path, root: &Path) -> Result<()> {
    if !candidate.starts_with(root) {
        anyhow::bail!(
            "TetherScript plugin path '{}' escapes workspace root '{}'",
            candidate.display(),
            root.display()
        );
    }
    if !is_tetherscript_source(candidate) {
        anyhow::bail!(
            "TetherScript plugin path '{}' must use the .tether extension or legacy .kl extension",
            candidate.display()
        );
    }
    Ok(())
}

fn is_tetherscript_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("tether" | "kl")
    )
}
