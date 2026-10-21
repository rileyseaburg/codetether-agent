//! Canonical local-file URI boundaries, including symlink escape rejection.
use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};
use url::Url;

fn local_path(uri: &str) -> Result<PathBuf> {
    let url = Url::parse(uri)?;
    ensure!(
        url.scheme() == "file" && url.query().is_none() && url.fragment().is_none(),
        "expected plain file URI"
    );
    ensure!(
        url.host_str()
            .is_none_or(|host| host.is_empty() || host == "localhost"),
        "remote file URI excluded"
    );
    url.to_file_path()
        .map_err(|_| anyhow::anyhow!("invalid local file URI"))
}

pub(super) fn workspace(uri: &str) -> Result<(PathBuf, String)> {
    let path = local_path(uri)?.canonicalize()?;
    ensure!(path.is_dir(), "workspace URI must identify a directory");
    let uri =
        Url::from_directory_path(&path).map_err(|_| anyhow::anyhow!("invalid workspace path"))?;
    Ok((path, uri.into()))
}

pub(super) fn descendant(root: &Path, uri: &str) -> Option<String> {
    let path = local_path(uri).ok()?;
    let path = resolve_missing(&path).ok()?;
    if path == root || !path.starts_with(root) || path.is_dir() {
        return None;
    }
    Url::from_file_path(path).ok().map(String::from)
}

fn resolve_missing(path: &Path) -> Result<PathBuf> {
    match path.canonicalize() {
        Ok(path) => Ok(path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            ensure!(
                !std::fs::symlink_metadata(path)
                    .is_ok_and(|metadata| metadata.file_type().is_symlink()),
                "dangling symlink excluded"
            );
            let name = path.file_name().context("invalid file path")?;
            let parent = path.parent().context("missing file ancestor")?;
            Ok(resolve_missing(parent)?.join(name))
        }
        Err(error) => Err(error.into()),
    }
}
