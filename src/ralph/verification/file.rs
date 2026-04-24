use anyhow::{Result, bail};
use std::path::Path;

pub fn exists(root: &Path, path: &str, is_glob: bool) -> Result<()> {
    if is_glob {
        if glob_exists(root, path)? {
            return Ok(());
        }
        bail!("no files matched glob `{path}`");
    }

    let candidate = root.join(path);
    if candidate.exists() {
        Ok(())
    } else {
        bail!("file `{}` does not exist", candidate.display())
    }
}

pub fn glob_exists(root: &Path, pattern: &str) -> Result<bool> {
    let anchored = if Path::new(pattern).is_absolute() {
        pattern.to_string()
    } else {
        root.join(pattern).to_string_lossy().to_string()
    };
    for entry in glob::glob(&anchored)? {
        if matches!(entry, Ok(path) if path.exists()) {
            return Ok(true);
        }
    }
    Ok(false)
}
