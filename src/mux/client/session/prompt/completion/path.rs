//! Expansion of user-entered completion paths.

use std::path::{Path, PathBuf};

pub(super) fn expand(argument: &str, workspace: &Path) -> PathBuf {
    if argument == "~"
        && let Some(home) = std::env::var_os("HOME")
    {
        return PathBuf::from(home);
    }
    if let Some(rest) = argument.strip_prefix("~/")
        && let Some(home) = std::env::var_os("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    let path = Path::new(argument);
    if path.is_absolute() {
        path.into()
    } else {
        workspace.join(path)
    }
}
