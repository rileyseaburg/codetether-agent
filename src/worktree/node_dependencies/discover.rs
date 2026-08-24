use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

pub(super) fn packages(source: &Path) -> Result<Vec<PathBuf>> {
    let args = ["ls-files", "-z", "--", "package.json", "**/package.json"];
    let output = crate::tool::git::process::output_blocking_refs(source, &args, false)?;
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
