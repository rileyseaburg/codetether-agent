use anyhow::{Result, bail};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub(super) fn packages(source: &Path) -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .current_dir(source)
        .args(["ls-files", "-z", "--", "package.json", "**/package.json"])
        .output()?;
    if !output.status.success() {
        bail!("failed to discover tracked package.json files");
    }
    let mut packages = output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| PathBuf::from(String::from_utf8_lossy(path).as_ref()))
        .filter_map(|path| path.parent().map(Path::to_path_buf))
        .collect::<Vec<_>>();
    packages.sort();
    packages.dedup();
    Ok(packages)
}
